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
                // Mengikuti saran compiler: tambahkan {}
                MainEvent::InitWindow {} => {
                    info!("GODICO: InitWindow event captured!");
                    // Di sini kita bisa coba akses window-nya
                    if let Some(window) = app.native_window() {
                         info!("GODICO: Window Width: {}", window.width());
                    }
                }
                _ => (),
            }
        }
    });
}
