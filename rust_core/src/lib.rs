#[no_mangle]
pub extern "C" fn JNI_OnLoad(
    _vm: *mut std::ffi::c_void,
    _reserved: *mut std::ffi::c_void
) -> i32 {
    65542
}

// Fungsi ini yang dicari oleh MainActivity.java
#[no_mangle]
pub extern "C" fn Java_com_godico_devhub_MainActivity_startIpcServer(
    _env: *mut std::ffi::c_void, 
    _class: *mut std::ffi::c_void
) {
    // Logic IPC Server lu di sini
}

#[no_mangle]
pub extern "C" fn android_main(app: *mut std::ffi::c_void) {
    // Logic untuk NativeActivity
}
