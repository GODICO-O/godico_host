use android_activity::{AndroidApp, AndroidAppMain, MainEvent, PollEvent};

#[no_mangle]
fn android_main(app: AndroidApp) {
    android_logger::init_once(android_logger::Config::default().with_tag("GODICO_NATIVE"));
    
    // API v0.5 lebih sederhana
    while let Some(event) = app.poll_events() {
        match event {
            PollEvent::Main(MainEvent::Resume) => {
                log::info!("GODICO NATIVE: Engine Aktif!");
            }
            PollEvent::Main(MainEvent::Destroy) => break,
            _ => (),
        }
    }
}
