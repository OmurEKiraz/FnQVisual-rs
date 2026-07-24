use crate::config::{AppConfig, EMBEDDED_BALANCED, EMBEDDED_PERFORMANCE, EMBEDDED_QUIET};
use crate::monitor::PowerMode;
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Box as GtkBox, Image, Label, Orientation, CssProvider};
use gtk4_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

pub struct AppState {
    pub window: ApplicationWindow,
    pub label: Label,
    pub image: Image,
    pub config: AppConfig,
    pub display_epoch: RefCell<u64>,
}

pub fn build_ui(app: &Application) -> Rc<AppState> {
    let config = crate::config::load_config();

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Fn+Q OSD")
        .default_width(config.window.width)
        .default_height(config.window.height)
        .build();

    // Katmanları ve Click-Through özelliğini kuruyoruz
    configure_window_layers(&window, &config);

    let container = GtkBox::new(Orientation::Vertical, 12);
    container.set_halign(gtk4::Align::Center);
    container.set_valign(gtk4::Align::Center);
    container.set_margin_start(24);
    container.set_margin_end(24);
    container.set_margin_top(20);
    container.set_margin_bottom(20);

    let image = Image::new();
    image.set_pixel_size(config.window.icon_size);

    let label = Label::new(None);
    label.set_use_markup(true);
    label.set_halign(gtk4::Align::Center);

    container.append(&image);
    container.append(&label);
    window.set_child(Some(&container));

    // TOML'dan gelen arka plan rengini CSS'e gömüyoruz
    let css_provider = CssProvider::new();
    let style_data = format!(
        "window {{ background-color: {}; border-radius: 24px; }}",
        config.window.background_rgba
    );
    css_provider.load_from_data(&style_data);

    if let Some(display) = gtk4::gdk::Display::default() {
        gtk4::style_context_add_provider_for_display(
            &display,
            &css_provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }

    // CLICK-THROUGH GARANTİSİ (Pencerenin tıklama alanını 0x0 yapıyoruz)
    window.connect_realize(|win| {
        let empty_region = gtk4::cairo::Region::create();
        if let Some(surface) = win.surface() {
            surface.set_input_region(&empty_region);
        }
    });

    Rc::new(AppState {
        window,
        label,
        image,
        config,
        display_epoch: RefCell::new(0),
    })
}

pub fn update_overlay(state: &Rc<AppState>, mode: PowerMode) {
    let target_cfg = match mode {
        PowerMode::Performance => &state.config.performance,
        PowerMode::Balanced => &state.config.balanced,
        PowerMode::LowPower => &state.config.quiet,
        PowerMode::Unknown => return,
    };

    // TOML'dan gelen yazı tipi, boyutu ve ağırlığı dinamik basılıyor
    state.label.set_markup(&format!(
        "<span size='{}' weight='{}' font_family='sans-serif' color='white'>{}</span>", 
        state.config.window.font_size,
        state.config.window.font_weight,
        target_cfg.text
    ));

    if let Some(ref path) = target_cfg.icon_path {
        state.image.set_from_file(Some(std::path::Path::new(path)));
    } else {
        let raw_bytes = match mode {
            PowerMode::Performance => EMBEDDED_PERFORMANCE,
            PowerMode::Balanced => EMBEDDED_BALANCED,
            PowerMode::LowPower => EMBEDDED_QUIET,
            _ => &[],
        };
        if let Ok(texture) = gtk4::gdk::Texture::from_bytes(&gtk4::glib::Bytes::from(raw_bytes)) {
            state.image.set_from_paintable(Some(&texture));
        }
    }

    state.window.present();

    let mut epoch = state.display_epoch.borrow_mut();
    *epoch += 1;
    let current_epoch = *epoch;

    let state_weak = Rc::downgrade(state);
    let duration = state.config.display_duration_ms;

    gtk4::glib::timeout_add_local(Duration::from_millis(duration), move || {
        if let Some(app_state) = state_weak.upgrade() {
            if *app_state.display_epoch.borrow() == current_epoch {
                app_state.window.set_visible(false);
            }
        }
        gtk4::glib::ControlFlow::Break
    });
}

fn configure_window_layers(window: &ApplicationWindow, config: &AppConfig) {
    let session_type = std::env::var("XDG_SESSION_TYPE").unwrap_or_default().to_lowercase();

    if session_type == "wayland" {
        window.init_layer_shell();
        window.set_layer(Layer::Overlay);
        window.set_namespace("fnq-overlay");
        window.set_keyboard_mode(KeyboardMode::None);

        // Konumlandırma Ayarları (TOML tabanlı)
        match config.window.anchor_edge.as_str() {
            "top" => {
                window.set_anchor(Edge::Top, true);
                window.set_margin(Edge::Top, config.window.margin_offset);
            },
            "bottom" => {
                window.set_anchor(Edge::Bottom, true);
                window.set_margin(Edge::Bottom, config.window.margin_offset);
            },
            _ => { // Center veya herhangi başka bir şeyde ekrana ortala
                window.set_anchor(Edge::Top, false);
                window.set_anchor(Edge::Bottom, false);
            }
        }
    } else {
        window.set_decorated(false);
        window.set_focusable(false);
        window.set_can_focus(false);
    }
}