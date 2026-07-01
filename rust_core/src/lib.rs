use jni::objects::JClass;
use jni::sys::jboolean;
use jni::JNIEnv;
use std::process::Command;

#[no_mangle]
pub extern "system" fn JNI_OnLoad(
    _vm: *mut std::ffi::c_void,
    _reserved: *mut std::ffi::c_void
) -> i32 {
    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(log::LevelFilter::Debug)
            .with_tag("GODICO_ROOT_DEBUG"),
    );
    65542
}

#[no_mangle]
pub extern "system" fn Java_com_godico_devhub_MainActivity_checkRootStatus(
    _env: JNIEnv, 
    _class: JClass
) -> jboolean {
    // Cek di beberapa lokasi umum biner SU
    let paths = ["/system/xbin/su", "/system/bin/su", "/sbin/su", "/vendor/bin/su"];
    
    for path in paths {
        log::debug!("Mencoba akses SU di: {}", path);
        let output = Command::new(path)
            .arg("-c")
            .arg("id")
            .output();

        if let Ok(out) = output {
            if out.status.success() {
                log::debug!("ROOT BERHASIL DETEKSI via {}", path);
                return 1;
            }
        }
    }
    
    log::debug!("ROOT GAGAL: Tidak ditemukan akses atau ditolak.");
    0
}
