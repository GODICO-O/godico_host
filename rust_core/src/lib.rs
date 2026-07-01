#[no_mangle]
pub extern "C" fn JNI_OnLoad(
    _vm: *mut std::ffi::c_void, 
    _reserved: *mut std::ffi::c_void
) -> i32 {
    // Memberitahu Android bahwa kita menggunakan JNI versi 1.6
    65536 // 0x00010006
}

// PENTING: Untuk GameActivity, kita butuh main entry point
// agar sistem Android tahu fungsi mana yang harus dijalankan
#[no_mangle]
pub extern "C" fn android_main(app: *mut std::ffi::c_void) {
    // Di sini nantinya kita panggil logic engine lu
}
