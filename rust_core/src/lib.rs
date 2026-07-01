use android_activity::{AndroidApp, MainEvent, PollEvent};
use log::info;

#[no_mangle]
fn android_main(app: AndroidApp) {
    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(log::LevelFilter::Info)
            .with_tag("GODICO_NATIVE")
    );

    info!("GODICO NATIVE: Engine Running...");

    app.poll_events(Some(std::time::Duration::from_millis(16)), |event| {
        if let PollEvent::Main(event) = event {
            match event {
                MainEvent::InitWindow => {
                    info!("GODICO: InitWindow event received.");
                }
                _ => (),
            }
        }
    });
}
