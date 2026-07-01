use android_activity::{AndroidApp, MainEvent, PollEvent};

#[no_mangle]
fn android_main(app: AndroidApp) {
    android_logger::init_once(android_logger::Config::default().with_tag("GODICO_NATIVE"));
    log::info!("GODICO NATIVE: Engine Starting...");

    app.poll_events(Some(std::time::Duration::from_millis(16)), |event| {
        match event {
            PollEvent::Main(event) => match event {
                // Semua event di v0.5.2 harus menggunakan sintaks { .. }
                MainEvent::InitWindow { .. } => {
                    log::info!("GODICO NATIVE: Window Ready");
                }
                MainEvent::Resume { .. } => {
                    log::info!("GODICO NATIVE: App Resumed");
                }
                MainEvent::Destroy => {
                    log::info!("GODICO NATIVE: Shutdown");
                }
                _ => (),
            },
            _ => (),
        }
    });
}
