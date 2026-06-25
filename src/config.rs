use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

pub const EMBEDDED_QUIET: &[u8] = include_bytes!("assets/quiet.png");
pub const EMBEDDED_BALANCED: &[u8] = include_bytes!("assets/balanced.png");
pub const EMBEDDED_PERFORMANCE: &[u8] = include_bytes!("assets/performance.png");

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModeConfig {
    pub text: String,
    pub icon_path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WindowConfig {
    pub width: i32,
    pub height: i32,
    pub icon_size: i32,
    pub font_size: i32,
    pub font_weight: String, // "heavy", "bold", "normal"
    pub anchor_edge: String, // "bottom", "top", "center"
    pub margin_offset: i32,  // Ekran kenarından kaç piksel içeride olacak
    pub background_rgba: String, // Örn: "rgba(25, 25, 25, 0.70)"
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub display_duration_ms: u64,
    pub window: WindowConfig,
    pub quiet: ModeConfig,
    pub balanced: ModeConfig,
    pub performance: ModeConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            display_duration_ms: 2500,
            window: WindowConfig {
                width: 200,
                height: 180,
                icon_size: 72,
                font_size: 16000, // GTK Pango ölçeğinde 16pt
                font_weight: "heavy".to_string(),
                anchor_edge: "bottom".to_string(),
                margin_offset: 100,
                background_rgba: "rgba(25, 25, 25, 0.70)".to_string(),
            },
            quiet: ModeConfig {
                text: "QUIET MODE".to_string(),
                icon_path: None,
            },
            balanced: ModeConfig {
                text: "BALANCED MODE".to_string(),
                icon_path: None,
            },
            performance: ModeConfig {
                text: "PERFORMANCE MODE".to_string(),
                icon_path: None,
            },
        }
    }
}

pub fn load_config() -> AppConfig {
    let config_dir = format!("{}/.config/fnq-visual", std::env::var("HOME").unwrap_or_default());
    let config_path = format!("{}/config.toml", config_dir);
    let path = Path::new(&config_path);

    if !path.exists() {
        let _ = fs::create_dir_all(config_dir);
        let default_toml = toml::to_string_pretty(&AppConfig::default()).unwrap();
        let _ = fs::write(path, default_toml);
        return AppConfig::default();
    }

    if let Ok(content) = fs::read_to_string(path) {
        if let Ok(cfg) = toml::from_str(&content) {
            return cfg;
        }
    }
    AppConfig::default()
}