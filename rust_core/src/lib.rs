use android_activity::{AndroidApp, MainEvent, PollEvent};
use std::thread;
use std::time::Duration;
use log::info;

#[no_mangle]
fn android_main(app: AndroidApp) {
    // Inisialisasi Logger
    android_logger::init_once(android_logger::Config::default().with_tag("GODICO_HACKER"));
    
    // Thread "Hacker Loop" - Berjalan di latar belakang
    thread::spawn(|| {
        let messages = [
            "[INITIALIZING_CORE_SEQUENCE...]",
            "[BYPASSING_FIREWALL_LAYER_7...]",
            "[ENCRYPTING_DATA_STREAM...]",
            "[SCANNING_LOCAL_NODES...]",
            "[UPLINK_ESTABLISHED_TO_VOID_NODE]",
            "[DEBUG_MODE_ACTIVE_NO_RETURN_PATH]",
            "[SYSTEM_ROOT_ACCESS_GRANTED]",
            "[VOID_DISPLAY_LINK_STABLE]",
        ];
        
        let mut i = 0;
        loop {
            info!("HACKER_STREAM -> {}", messages[i % messages.len()]);
            i += 1;
            thread::sleep(Duration::from_millis(800));
        }
    });

    info!("GODICO NATIVE: Engine Starting...");
    
    // Event loop utama agar aplikasi tetap responsif
    app.poll_events(Some(Duration::from_millis(16)), |event| {
        match event {
            PollEvent::Main(event) => match event {
                MainEvent::InitWindow { .. } => info!("GODICO: Window Ready"),
                MainEvent::Resume { .. } => info!("GODICO: App Resumed"),
                _ => (),
            },
            _ => (),
        }
    });
}
