use android_activity::{AndroidApp, MainEvent, PollEvent};

#[no_mangle]
fn android_main(app: AndroidApp) {
    android_logger::init_once(android_logger::Config::default().with_tag("GODICO_NATIVE"));
    
    // Versi 0.5.2 mewajibkan closure (callback) di dalam poll_events
    app.poll_events(Some(std::time::Duration::from_millis(16)), |event| {
        match event {
            PollEvent::Main(event) => match event {
                // Gunakan { .. } untuk mengabaikan data internal yang tidak kita pakai
                MainEvent::Resume { .. } => {
                    log::info!("GODICO NATIVE: Engine Aktif!");
                }
                MainEvent::Destroy => {
                    log::info!("GODICO NATIVE: Engine Dimatikan");
                }
                _ => (),
            },
            _ => (),
        }
    });
}
