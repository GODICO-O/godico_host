use android_activity::{AndroidApp, MainEvent, PollEvent};
use log::info;
use pixels::{Pixels, SurfaceTexture};
use winit::event_loop::EventLoopBuilder;

#[no_mangle]
fn android_main(app: AndroidApp) {
    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(log::LevelFilter::Info)
            .with_tag("GODICO_NATIVE")
    );

    info!("GODICO NATIVE: Engine Initializing Pixel Pipeline...");

    app.poll_events(Some(std::time::Duration::from_millis(16)), |event| {
        match event {
            PollEvent::Main(event) => match event {
                MainEvent::InitWindow { window } => {
                    info!("GODICO: Rendering Pipeline Ready for Injection.");
                    // Di sini nanti kita set up Pixels dengan Native Window
                    // window adalah pointer ke ANativeWindow
                },
                MainEvent::Destroy => {
                    info!("GODICO: Engine Destroyed.");
                },
                _ => (),
            },
            _ => (),
        }
    });
}
