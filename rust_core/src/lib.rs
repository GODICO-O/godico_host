use std::ffi::c_void;
use std::thread;
use std::time::Duration;
use log::info;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_godico_devhub_MainActivity_startIpcServer(
    _env: *mut c_void, _jclass: *mut c_void,
) {
    // Logger dengan filter yang aman
    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(log::LevelFilter::Info)
            .with_tag("GODICO_HACKER")
    );

    info!("JNI: Engine Aktif! Memulai Loop Hacker...");

    // Spawn thread agar tidak memblokir UI Thread dari MainActivity
    thread::spawn(|| {
        let messages = [
            "[INITIALIZING_CORE...]",
            "[BYPASSING_FIREWALL...]",
            "[ENCRYPTING_DATA...]",
            "[UPLINK_STABLE...]",
            "[VOID_NODE_SYNCED]",
        ];
        
        let mut i = 0;
        loop {
            info!("HACKER_STREAM -> {}", messages[i % messages.len()]);
            i += 1;
            // Delay 800ms cukup untuk estetika tanpa membebani CPU
            thread::sleep(Duration::from_millis(800));
        }
    });
}
