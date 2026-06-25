use notify::{Watcher, RecursiveMode};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::time::Duration;
use tokio::sync::mpsc;

const PROFILE_PATH: &str = "/sys/firmware/acpi/platform_profile";

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PowerMode {
    Performance,
    Balanced,
    LowPower,
    Unknown,
}

impl PowerMode {
    pub fn from_str(s: &str) -> Self {
        match s {
            "performance" => Self::Performance,
            "balanced" => Self::Balanced,
            "low-power" => Self::LowPower,
            _ => Self::Unknown,
        }
    }
}

pub fn read_kernel_profile() -> PowerMode {
    let mut file = match File::open(PROFILE_PATH) {
        Ok(f) => f,
        Err(_) => return PowerMode::Unknown,
    };
    let mut buffer = [0; 32];
    if let Ok(bytes_read) = file.read(&mut buffer) {
        let content = String::from_utf8_lossy(&buffer[..bytes_read]);
        return PowerMode::from_str(content.trim());
    }
    PowerMode::Unknown
}

pub async fn start_kernel_monitor(tx: mpsc::Sender<PowerMode>) {
    let (notify_tx, mut notify_rx) = mpsc::channel(1);
    
    let mut watcher = notify::recommended_watcher(move |res: Result<notify::Event, _>| {
        if let Ok(event) = res {
            if event.kind.is_modify() {
                let _ = notify_tx.blocking_send(());
            }
        }
    }).unwrap();

    let _ = watcher.watch(Path::new(PROFILE_PATH), RecursiveMode::NonRecursive);

    while let Some(_) = notify_rx.recv().await {
        // Debounce settle duration to prevent double triggers
        tokio::time::sleep(Duration::from_millis(40)).await;
        let current_mode = read_kernel_profile();
        let _ = tx.send(current_mode).await;
    }
}