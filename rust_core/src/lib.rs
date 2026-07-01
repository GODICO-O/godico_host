use std::ffi::c_void;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_godico_devhub_MainActivity_startIpcServer(
    _env: *mut c_void, _jclass: *mut c_void,
) {
    // Log sederhana untuk memastikan Rust terpanggil
    android_logger::init_once(android_logger::Config::default().with_tag("GODICO_JNI"));
    log::info!("JNI: Rust Engine Terpanggil dengan Sukses!");
}
