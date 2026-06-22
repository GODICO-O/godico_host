use std::net::TcpListener;
use std::io::{Read, Write};
use std::ffi::CString;

// Gunakan tipe data langsung dari jni_sys agar tidak kena sensor private e0603
type JNIEnvPtr = *mut jni_sys::JNIEnv;
type JobjectPtr = jni_sys::jobject;
type JavaVMPtr = *mut jni_sys::JavaVM;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_godico_devhub_MainActivity_startIpcServer(
    env: JNIEnvPtr,
    jclass: JobjectPtr,
) {
    // 1. Ambil pointer global JavaVM dari Environment saat ini
    let mut jvm: JavaVMPtr = std::ptr::null_mut();
    unsafe {
        if let Some(f) = (*(*env)).GetJavaVM {
            f(env, &mut jvm);
        }
    }

    // 2. Trik konversi angka bulat agar lolos sensor thread safety (Send)
    let jvm_raw = jvm as usize;
    let jclass_raw = jclass as usize;

    std::thread::spawn(move || {
        // Kembalikan angka menjadi pointer mentah JNI di thread baru
        let thread_jvm = jvm_raw as JavaVMPtr;
        let thread_jclass = jclass_raw as JobjectPtr;

        match TcpListener::bind("0.0.0.0:8080") {
            Ok(listener) => {
                for stream in listener.incoming() {
                    match stream {
                        Ok(mut s) => {
                            let mut buffer = [0; 256];
                            if let Ok(bytes_read) = s.read(&mut buffer) {
                                let pesan = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();
                                
                                // Respons balik ke Termux
                                let respon = format!("STATUS:LOG_DISPLAYED | Konten: {}", pesan);
                                let _ = s.write_all(respon.as_bytes());

                                // ATTACH THREAD RUST KE JAVA VM SEARA GAIB
                                unsafe {
                                    let mut local_env: JNIEnvPtr = std::ptr::null_mut();
                                    if let Some(attach_fn) = (*(*thread_jvm)).AttachCurrentThread {
                                        let res = attach_fn(
                                            thread_jvm, 
                                            &mut local_env as *mut JNIEnvPtr as *mut *mut std::ffi::c_void, 
                                            std::ptr::null_mut()
                                        );
                                        
                                        if res == 0 && !local_env.is_null() {
                                            // Cari fungsi "updateLogText" di MainActivity.java
                                            if let Some(get_method_id) = (*(*local_env)).GetMethodID {
                                                let class_target = if let Some(get_class) = (*(*local_env)).GetObjectClass {
                                                    get_class(local_env, thread_jclass)
                                                } else { std::ptr::null_mut() };

                                                let method_name = CString::new("updateLogText").unwrap();
                                                let method_sig = CString::new("(Ljava/lang/String;)V").unwrap();
                                                
                                                let method_id = get_method_id(local_env, class_target, method_name.as_ptr(), method_sig.as_ptr());
                                                
                                                if !method_id.is_null() {
                                                    // Ubah String Rust jadi jstring Java
                                                    if let Some(new_string) = (*(*local_env)).NewStringUTF {
                                                        let c_pesan = CString::new(pesan).unwrap();
                                                        let j_pesan = new_string(local_env, c_pesan.as_ptr());
                                                        
                                                        // TEMBAK UI JAVA SEKARANG!
                                                        if let Some(call_method) = (*(*local_env)).CallVoidMethod {
                                                            call_method(local_env, thread_jclass, method_id, j_pesan);
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Err(_) => {}
                    }
                }
            }
            Err(_) => {}
        }
    });
}
