use android_activity::{AndroidApp, MainEvent, PollEvent};
use std::thread;
use std::time::Duration;
use log::info;

#[no_mangle]
fn android_main(app: AndroidApp) {
    // 1. Pastikan logger inisialisasi di baris pertama
    android_logger::init_once(
        android_logger::Config::default()
            .with_min_level(log::Level::Info)
            .with_tag("GODICO_HACKER")
    );
    
    info!("GODICO: Engine Initializing...");

    // 2. Kita gunakan flag agar thread hacker tidak jalan sebelum aplikasi ready
    app.poll_events(Some(Duration::from_millis(16)), |event| {
        match event {
            PollEvent::Main(event) => match event {
                MainEvent::Resume { .. } => {
                    info!("GODICO: App Resumed - Starting Hacker Thread");
                    
                    // Kita spawn thread HANYA setelah aplikasi benar-benar aktif
                    thread::spawn(|| {
                        let messages = [
                            "[INITIALIZING_CORE...]",
                            "[BYPASSING_FIREWALL...]",
                            "[ENCRYPTING_DATA...]",
                            "[UPLINK_STABLE...]",
                        ];
                        let mut i = 0;
                        loop {
                            info!("HACKER_STREAM -> {}", messages[i % messages.len()]);
                            i += 1;
                            thread::sleep(Duration::from_millis(800));
                        }
                    });
                }
                MainEvent::Destroy => {
                    info!("GODICO: Shutdown");
                }
                _ => (),
            },
            _ => (),
        }
    });
}
