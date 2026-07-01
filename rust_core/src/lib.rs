#[no_mangle]
pub extern "C" fn JNI_OnLoad(
    _vm: *mut std::ffi::c_void,
    _reserved: *mut std::ffi::c_void
) -> i32 {
    65542
}

#[no_mangle]
pub extern "C" fn Java_com_godico_devhub_MainActivity_startIpcServer(
    _env: *mut std::ffi::c_void, 
    _class: *mut std::ffi::c_void
) {
    // Logic IPC Server
}

#[no_mangle]
pub extern "C" fn android_main(app: *mut std::ffi::c_void) {
    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(log::LevelFilter::Debug)
            .with_tag("GODICO_NATIVE"),
    );

    log::debug!("Engine Native menyala, menunggu event sistem...");

    // Catatan: Ini adalah konsep loop untuk Native Activity.
    // Jika lu menggunakan crate 'android-activity' (rekomendasi), 
    // sistem event ini sudah dihandle otomatis oleh macro.
    
    // Logika dasar untuk menangani orientasi layar:
    // 1. Dapatkan native window.
    // 2. Pantau APP_CMD_CONFIG_CHANGED melalui event loop.
    // 3. Update ukuran buffer render saat event diterima.
}
