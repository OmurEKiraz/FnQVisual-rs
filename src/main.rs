mod config;
mod monitor;
mod ui;

use gtk4::prelude::*;
use gtk4::Application;

fn main() {
    let app = Application::builder()
        .application_id("org.omrkrz.fnqvisual")
        .build();

    app.connect_activate(move |application| {
        // Arayüzü oluştur
        let app_state = ui::build_ui(application);
        
        // GTK'nın eski Sender'ı yerine inanılmaz hafif async_channel kullanıyoruz
        let (tx, rx) = async_channel::unbounded();

        // Kernel'ı dinleyen hafif thread'i başlatıyoruz
        monitor::start_kernel_monitor(tx);

        // Arayüzü anlık güncellemek için GTK'nın yerel asenkron motorunu kullanıyoruz
        // CPU kullanımı: %0 (Sadece mesaj gelince uyanır)
        gtk4::glib::MainContext::default().spawn_local(async move {
            while let Ok(mode) = rx.recv().await {
                ui::update_overlay(&app_state, mode);
            }
        });
    });

    app.run_with_args(&[""]);
}