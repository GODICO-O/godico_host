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
    // Logic IPC Server di sini
}

#[no_mangle]
pub extern "system" fn Java_com_godico_devhub_MainActivity_checkRootStatus(
    _env: JNIEnv, 
    _class: JClass
) -> jboolean {
    let output = Command::new("su")
        .arg("-c")
        .arg("id")
        .output();

    match output {
        Ok(out) => if out.status.success() { 1 } else { 0 },
        Err(_) => 0,
    }
}
