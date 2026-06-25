use gtk4::prelude::*;
use gtk4::ApplicationWindow;
use gtk4_layer_shell::{Layer, LayerShell, Edge, KeyboardMode};

pub fn configure_window_layers(window: &ApplicationWindow) {
    let session_type = std::env::var("XDG_SESSION_TYPE").unwrap_or_default().to_lowercase();

    if session_type == "wayland" {
        window.init_layer_shell();
        window.set_layer(Layer::Overlay);
        window.set_namespace("fnq-overlay");

        window.set_margin(Edge::Top, 12);
        window.set_anchor(Edge::Top, true);
        window.set_anchor(Edge::Bottom, false);
        window.set_anchor(Edge::Left, false);
        window.set_anchor(Edge::Right, false);

        window.set_keyboard_mode(KeyboardMode::None);
    } else {
        window.set_decorated(false);
        window.set_focusable(false);
    }
}