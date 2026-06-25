use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

// Default hardcoded SVGs if custom paths are not provided
pub const DEFAULT_QUIET_SVG: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="white"><path d="M12 2A10 10 0 0 0 2 12a10 10 0 0 0 10 10 10 10 0 0 0 10-10A10 10 0 0 0 12 2zm0 2a8 8 0 0 1 8 8 8 8 0 0 1-8 8 8 8 0 0 1-8-8 8 8 0 0 1 8-8zm-1 3v6h2V7zm0 8v2h2v-2z"/></svg>"#;
pub const DEFAULT_BALANCED_SVG: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="white"><path d="M12 2L2 22h20L12 2zm0 3.99L19.53 19H4.47L12 5.99zM11 10v4h2v-4h-2zm0 6v2h2v-2h-2z"/></svg>"#;
pub const DEFAULT_PERFORMANCE_SVG: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="white"><path d="M13.13 2.18a1 1 0 0 0-1.26 0L4.31 8.2A1 1 0 0 0 4 9.2c0 2.22 1 4.54 2.8 6.64A13 13 0 0 0 12 21.8a13 13 0 0 0 5.2-5.96c1.8-2.1 2.8-4.42 2.8-6.64a1 1 0 0 0-.31-.74l-6.56-6.28zM12 4.14L17.44 9.3c.06 1.76-.73 3.65-2.22 5.39A11.14 11.14 0 0 1 12 19.66a11.14 11.14 0 0 1-3.22-4.97c-1.5-1.74-2.28-3.63-2.22-5.39L12 4.14z"/></svg>"#;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModeConfig {
    pub text: String,
    pub bg_color: String,
    pub icon_path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub display_duration_ms: u64,
    pub quiet: ModeConfig,
    pub balanced: ModeConfig,
    pub performance: ModeConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            display_duration_ms: 2500,
            quiet: ModeConfig {
                text: "QUIET MODE".to_string(),
                bg_color: "#28A65A".to_string(),
                icon_path: None,
            },
            balanced: ModeConfig {
                text: "BALANCED MODE".to_string(),
                bg_color: "#2874C2".to_string(),
                icon_path: None,
            },
            performance: ModeConfig {
                text: "PERFORMANCE MODE".to_string(),
                bg_color: "#C22828".to_string(),
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
        if let Ok(parsed) = toml::from_str(&content) {
            return parsed;
        }
    }
    AppConfig::default()
}