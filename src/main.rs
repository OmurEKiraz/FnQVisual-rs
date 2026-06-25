mod config;
mod monitor;
mod ui;

use gtk4::prelude::*;
use gtk4::Application;
use std::path::Path;
use std::process;

// Pre-flight health check to prevent crashes on unsupported hardware
fn pre_flight_check() {
    let profile_path = "/sys/firmware/acpi/platform_profile";
    
    if !Path::new(profile_path).exists() {
        eprintln!("[ERROR] Lenovo ACPI platform_profile not found!");
        eprintln!("This device might not be supported, or the required kernel module is missing.");
        process::exit(1); // Graceful exit
    }

    let session = std::env::var("XDG_SESSION_TYPE").unwrap_or_else(|_| "Unknown".to_string());
    println!("[INFO] Starting FnQVisual...");
    println!("[INFO] Display Server: {}", session.to_uppercase());
}

fn main() {
    // Run hardware and session checks before rendering the UI
    pre_flight_check();

    let app = Application::builder()
        .application_id("org.omrkrz.fnqvisual")
        .build();

    app.connect_activate(move |application| {
        // Build the UI state
        let app_state = ui::build_ui(application);
        
        // Using an ultra-lightweight async channel instead of GTK's native sender
        let (tx, rx) = async_channel::unbounded();

        // Start the background thread listening to kernel ACPI events
        monitor::start_kernel_monitor(tx);

        // Spawn a local async task in GTK's main context to update the UI
        // CPU Usage: 0% (Only wakes up when a signal is received)
        gtk4::glib::MainContext::default().spawn_local(async move {
            while let Ok(mode) = rx.recv().await {
                ui::update_overlay(&app_state, mode);
            }
        });
    });

    app.run_with_args(&[""]);
}