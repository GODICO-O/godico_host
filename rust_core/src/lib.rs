use jni::objects::JClass;
use jni::sys::jboolean;
use jni::JNIEnv;
use std::process::Command;

#[no_mangle]
pub extern "system" fn JNI_OnLoad(
    _vm: *mut std::ffi::c_void,
    _reserved: *mut std::ffi::c_void
) -> i32 {
    65542
}

#[no_mangle]
pub extern "system" fn Java_com_godico_devhub_MainActivity_startIpcServer(
    _env: JNIEnv, 
    _class: JClass
) {
    // Logic IPC Server
}

#[no_mangle]
pub extern "system" fn Java_com_godico_devhub_MainActivity_checkRootStatus(
    _env: JNIEnv, 
    _class: JClass
) -> jboolean {
    // Gunakan full path untuk menghindari ambiguitas shell
    // Pakai clear_env() agar su tidak mewarisi environment yang bikin crash
    let output = Command::new("/system/bin/su")
        .arg("-c")
        .arg("id")
        .env_clear() 
        .output();

    match output {
        Ok(out) => {
            if out.status.success() { 1 } else { 0 }
        },
        Err(_) => 0,
    }
}
