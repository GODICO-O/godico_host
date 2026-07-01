#[no_mangle]
pub extern "C" fn JNI_OnLoad(
    _vm: *mut std::ffi::c_void,
    _reserved: *mut std::ffi::c_void
) -> i32 {
    // 65542 adalah JNI_VERSION_1_6, nilai wajib agar Android tidak crash
    65542
}

#[no_mangle]
pub extern "C" fn android_main(app: *mut std::ffi::c_void) {
    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(log::LevelFilter::Debug)
            .with_tag("GODICO_NATIVE"),
    );
    
    log::debug!("Engine Native sudah menyala, Komandan!");
    
    // Logika engine lu akan berjalan di sini
}
