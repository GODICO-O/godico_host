use std::net::TcpListener;
use std::io::{Read, Write};
use std::ffi::CString;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_godico_devhub_MainActivity_startIpcServer(
    env: *mut ndk_sys::JNIEnv,
    jclass: ndk_sys::jobject,
) {
    // Karena env dan jclass akan dipakai di dalam thread terpisah, kita butuh arsitektur JavaVM.
    // Namun untuk implementasi super stabil, ringan, dan anti-crash di Oppo, kita buat trik duplikasi Environment
    // atau menggunakan pointer mentah yang dilempar secara aman ke global runtime jika diperlukan.
    // Kali ini kita gunakan taktik mutlak: Simpan JavaVM secara global atau panggil balik langsung saat stream masuk!
    
    // Kita ambil pointer global JVM agar thread Rust bisa berinteraksi dengan UI Java kapan saja
    let mut jvm: *mut ndk_sys::JavaVM = std::ptr::null_mut();
    unsafe {
        if let Some(f) = (*(*env)).GetJavaVM {
            f(env, &mut jvm);
        }
    }

    std::thread::spawn(move || {
        match TcpListener::bind("0.0.0.0:8080") {
            Ok(listener) => {
                for stream in listener.incoming() {
                    match stream {
                        Ok(mut s) => {
                            let mut buffer = [0; 256];
                            if let Ok(bytes_read) = s.read(&mut buffer) {
                                let pesan = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();
                                
                                // Siapkan respons balik ke Termux
                                let respon = format!("STATUS:LOG_DISPLAYED | Konten: {}", pesan);
                                let _ = s.write_all(respon.as_bytes());

                                // ATTACH THREAD RUST KE JAVA VM SEARA GAIB
                                unsafe {
                                    let mut local_env: *mut ndk_sys::JNIEnv = std::ptr::null_mut();
                                    if let Some(attach_fn) = (*(*jvm)).AttachCurrentThread {
                                        let res = attach_fn(jvm, &mut local_env as *mut *mut ndk_sys::JNIEnv as *mut *mut std::ffi::c_void, std::ptr::null_mut());
                                        if res == 0 && !local_env.is_null() {
                                            
                                            // Cari fungsi "updateLogText" di dalam MainActivity.java
                                            if let Some(get_method_id) = (*(*local_env)).GetMethodID {
                                                let class_target = if let Some(get_class) = (*(*local_env)).GetObjectClass {
                                                    get_class(local_env, jclass)
                                                } else { std::ptr::null_mut() };

                                                let method_name = CString::new("updateLogText").unwrap();
                                                let method_sig = CString::new("(Ljava/lang/String;)V").unwrap();
                                                
                                                let method_id = get_method_id(local_env, class_target, method_name.as_ptr(), method_sig.as_ptr());
                                                
                                                if !method_id.is_null() {
                                                    // Ubah String Rust menjadi jstring Java
                                                    if let Some(new_string) = (*(*local_env)).NewStringUTF {
                                                        let c_pesan = CString::new(pesan).unwrap();
                                                        let j_pesan = new_string(local_env, c_pesan.as_ptr());
                                                        
                                                        // PANGGIL BALIK JAVA SEKARANG!
                                                        if let Some(call_method) = (*(*local_env)).CallVoidMethod {
                                                            call_method(local_env, jclass, method_id, j_pesan);
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
