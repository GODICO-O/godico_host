use android_activity::{AndroidApp, MainEvent, PollEvent};

#[no_mangle]
fn android_main(app: AndroidApp) {
    android_logger::init_once(android_logger::Config::default().with_tag("GODICO_NATIVE"));
    log::info!("GODICO NATIVE: Engine Starting...");

    // Event Loop yang efisien dan smooth
    app.poll_events(Some(std::time::Duration::from_millis(10)), |event| {
        match event {
            PollEvent::Main(event) => match event {
                MainEvent::InitWindow => {
                    log::info!("GODICO NATIVE: Window Ready - Render Start");
                }
                MainEvent::Resume { .. } => {
                    log::info!("GODICO NATIVE: App Resumed - Smooth Flow");
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
