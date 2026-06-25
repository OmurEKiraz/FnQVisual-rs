use notify::{Watcher, RecursiveMode, EventKind};
use std::fs;
use std::path::Path;
use std::time::Duration;
use std::thread;

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
        match s.trim() {
            "performance" => Self::Performance,
            "balanced" => Self::Balanced,
            "low-power" => Self::LowPower,
            _ => Self::Unknown,
        }
    }
}

pub fn read_kernel_profile() -> PowerMode {
    if let Ok(content) = fs::read_to_string(PROFILE_PATH) {
        return PowerMode::from_str(&content);
    }
    PowerMode::Unknown
}

pub fn start_kernel_monitor(sender: async_channel::Sender<PowerMode>) {
    thread::spawn(move || {
        let (notify_tx, notify_rx) = std::sync::mpsc::channel();
        
        let mut watcher = notify::recommended_watcher(move |res: Result<notify::Event, _>| {
            if let Ok(event) = res {
                if matches!(event.kind, EventKind::Modify(_)) {
                    let _ = notify_tx.send(());
                }
            }
        }).unwrap();

        let _ = watcher.watch(Path::new(PROFILE_PATH), RecursiveMode::NonRecursive);
        
        let mut last_mode = PowerMode::Unknown;

        // ÇÖZÜM BURADA: `for` yerine `while let` kullanıyoruz. 
        // Böylece notify_rx'in sahipliği bizde kalıyor ve içeride try_recv yapabiliyoruz.
        while let Ok(_) = notify_rx.recv() {
            // Spam (Kasma) Koruması: Donanımdan gelen art arda sinyalleri birleştirmek için ufak bir mola
            thread::sleep(Duration::from_millis(150)); 
            
            // Kuyrukta biriken diğer gereksiz sinyalleri çöp kutusuna atıyoruz
            while let Ok(_) = notify_rx.try_recv() {}

            let current_mode = read_kernel_profile();
            
            // Sadece mod gerçekten değiştiyse ve okunabilir durumdaysa ekrana bas
            if current_mode != last_mode && current_mode != PowerMode::Unknown {
                if sender.send_blocking(current_mode).is_err() {
                    break; 
                }
                last_mode = current_mode;
            }
        }
    });
}