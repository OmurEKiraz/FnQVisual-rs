mod config;
mod monitor;

use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Box as GtkBox, Label, Orientation, Image};
use gtk4_layer_shell::{Layer, LayerShell, Anchor};
use monitor::PowerMode;
use std::cell::RefCell;
use std::rc::Rc;
use tokio::sync::mpsc;

struct AppState {
    window: ApplicationWindow,
    label: Label,
    image: Image,
    config: config::AppConfig,
}

fn build_ui(app: &Application, rx: mpsc::Receiver<PowerMode>) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Fn+Q Visual Overlay")
        .build();

    // 🌟 THE WAYLAND HACK: Force window to behave as a top system overlay
    window.init_layer_shell();
    window.set_layer(Layer::Overlay);
    window.set_namespace("fnq-overlay");

    // Connect window edge anchors
    window.set_anchor(Anchor::Top, true);
    window.set_anchor(Anchor::Bottom, false);
    window.set_anchor(Anchor::Left, false);
    window.set_anchor(Anchor::Right, false);

    // Completely bypass mouse input and keyboard traps (Ghost Mode)
    window.set_keyboard_mode(gtk4_layer_shell::KeyboardMode::None);
    
    let container = GtkBox::new(Orientation::Horizontal, 12);
    container.set_margin_start(16);
    container.set_margin_end(16);
    container.set_margin_top(6);
    container.set_margin_bottom(6);

    let image = Image::new();
    let label = Label::new(None);
    
    // Inject Pango markup optimization into text rendering styling
    label.set_use_markup(true);

    container.append(&image);
    container.append(&label);
    window.set_child(Some(&container));

    let app_config = config::load_config();
    let state = Rc::new(RefCell::new(AppState {
        window,
        label,
        image,
        config: app_config,
    }));

    // Trigger state handler to listen to async monitor channel tokens within the GLib/GTK main loop
    let main_context = glib::MainContext::default();
    let state_clone = Rc::clone(&state);
    
    main_context.spawn_local(async move {
        let mut receiver = rx;
        while let Some(mode) = receiver.recv().await {
            update_overlay(&state_clone, mode);
        }
    });
}

fn update_overlay(state_rc: &Rc<RefCell<AppState>>, mode: PowerMode) {
    let mut state = state_rc.borrow_mut();
    let cfg = &state.config;

    let target_cfg = match mode {
        PowerMode::Performance => &cfg.performance,
        PowerMode::Balanced => &cfg.balanced,
        PowerMode::LowPower => &cfg.quiet,
        PowerMode::Unknown => return,
    };

    // Style layout color and background using pure CSS provider definitions
    let css_provider = gtk4::CssProvider::new();
    let style_data = format!(
        "window {{ background-color: {}; border-radius: 8px; color: white; font-weight: bold; }}",
        target_cfg.bg_color
    );
    css_provider.load_from_data(&style_data);
    
    if let Some(display) = gdk4::Display::default() {
        gtk4::style_context_add_provider_for_display(
            &display,
            &css_provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }

    state.label.set_markup(&format!("<span size='11000'>{}</span>", target_cfg.text));

    // Handle Custom Icon Paths or fall back onto internal compiled raw SVGs
    if let Some(ref path) = target_cfg.icon_path {
        state.image.set_from_file(Some(path));
    } else {
        let svg_raw = match mode {
            PowerMode::Performance => config::DEFAULT_PERFORMANCE_SVG,
            PowerMode::Balanced => config::DEFAULT_BALANCED_SVG,
            PowerMode::LowPower => config::DEFAULT_QUIET_SVG,
            _ => "",
        };
        let stream = gio::MemoryInputStream::from_bytes(&glib::Bytes::from(svg_raw.as_bytes()));
        if let Ok(pixbuf) = gdk4_pixbuf::Pixbuf::from_stream_at_scale(&stream, 20, 20, true, gio::Cancellable::NONE) {
            state.image.set_from_pixbuf(Some(&pixbuf));
        }
    }

    // Wake window up and render top overlay smoothly
    state.window.present();

    // Auto-hide timer execution using standard non-blocking GLib futures timeouts
    let window_weak = state.window.downgrade();
    let duration = cfg.display_duration_ms;
    
    glib::timeout_add_local(Duration::from_millis(duration), move || {
        if let Some(window) = window_weak.upgrade() {
            window.set_visible(false);
        }
        glib::ControlFlow::Break
    });
}

fn main() {
    let (tx, rx) = mpsc::channel::<PowerMode>(10);

    // Initial check to prevent blank execution
    let initial_mode = monitor::read_kernel_profile();
    let _ = tx.blocking_send(initial_mode);

    // Start background file notification thread
    tokio::runtime::Runtime::new().unwrap().spawn(async move {
        monitor::start_kernel_monitor(tx).await;
    });

    let app = Application::builder()
        .application_id("org.omrkrz.fnqvisual")
        .build();

    app.connect_activate(move |application| {
        // Trick receiver channel boundary ownership matching by setting up an internal unsafe shallow twin swap
        let ptr = &rx as *const mpsc::Receiver<PowerMode> as *mut mpsc::Receiver<PowerMode>;
        let unsafe_rx = unsafe { std::ptr::read(ptr) };
        build_ui(application, unsafe_rx);
    });

    app.run_with_args(&[""]);
}