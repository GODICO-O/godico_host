use jni::objects::JClass;
use jni::sys::jboolean;
use jni::JNIEnv;
use std::process::Command;
use std::io::ErrorKind;

#[no_mangle]
pub extern "system" fn Java_com_godico_devhub_MainActivity_checkRootStatus(
    _env: JNIEnv, 
    _class: JClass
) -> jboolean {
    // Meniru logika Shell.SU.available()
    // Kita coba cek apakah biner 'su' bisa dijalankan tanpa memblokir thread utama
    match Command::new("su").arg("-c").arg("exit").status() {
        Ok(status) => if status.success() { 1 } else { 0 },
        Err(e) => {
            // Kalau error karena perintah tidak ditemukan, return false
            if e.kind() == ErrorKind::NotFound { 0 } else { 0 }
        }
    }
}
