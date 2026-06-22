use std::net::TcpStream;
use std::io::{BufReader, BufRead};
use std::ffi::CString;
use std::time::Duration;

type JNIEnvPtr = *mut jni_sys::JNIEnv;
type JobjectPtr = jni_sys::jobject;
type JavaVMPtr = *mut jni_sys::JavaVM;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_godico_devhub_MainActivity_startIpcServer(
    env: JNIEnvPtr,
    jclass: JobjectPtr,
) {
    let mut jvm: JavaVMPtr = std::ptr::null_mut();
    unsafe {
        if let Some(f) = (*(*env)).GetJavaVM {
            f(env, &mut jvm);
        }
    }

    let jvm_raw = jvm as usize;
    let jclass_raw = jclass as usize;

    std::thread::spawn(move || {
        let thread_jvm = jvm_raw as JavaVMPtr;
        let thread_jclass = jclass_raw as JobjectPtr;

        loop {
            // Gunakan IP WLAN HP Lu (Sesuaikan jika IP berubah!)
            if let Ok(stream) = TcpStream::connect("192.168.0.110:8080") {
                // Bungkus stream dengan BufReader agar bisa mendeteksi baris baru (\n) hasil ketikan ncat secara instan!
                let mut reader = BufReader::new(stream);
                let mut baris_teks = String::new();
                
                // Baca baris per baris secara real-time setiap tombol Enter ditekan di Termux
                while let Ok(bytes) = reader.read_line(&mut baris_teks) {
                    if bytes == 0 { break; } // Koneksi terputus
                    
                    let pesan = baris_teks.trim().to_string();
                    baris_teks.clear(); // Kosongkan penampung untuk ketikan berikutnya
                    
                    if pesan.is_empty() { continue; }

                    // Eksekusi pelemparan data biner ke UI Java via JNI
                    unsafe {
                        let mut local_env: JNIEnvPtr = std::ptr::null_mut();
                        if let Some(attach_fn) = (*(*thread_jvm)).AttachCurrentThread {
                            let res = attach_fn(
                                thread_jvm, 
                                &mut local_env as *mut JNIEnvPtr as *mut *mut std::ffi::c_void, 
                                std::ptr::null_mut()
                            );
                            
                            if res == 0 && !local_env.is_null() {
                                if let Some(get_method_id) = (*(*local_env)).GetMethodID {
                                    let class_target = if let Some(get_class) = (*(*local_env)).GetObjectClass {
                                        get_class(local_env, thread_jclass)
                                    } else { std::ptr::null_mut() };

                                    let method_name = CString::new("updateLogText").unwrap();
                                    let method_sig = CString::new("(Ljava/lang/String;)V").unwrap();
                                    
                                    let method_id = get_method_id(local_env, class_target, method_name.as_ptr(), method_sig.as_ptr());
                                    
                                    if !method_id.is_null() {
                                        if let Some(new_string) = (*(*local_env)).NewStringUTF {
                                            let c_pesan = CString::new(pesan.clone()).unwrap();
                                            let j_pesan = new_string(local_env, c_pesan.as_ptr());
                                            
                                            if let Some(call_method) = (*(*local_env)).CallVoidMethod {
                                                call_method(local_env, thread_jclass, method_id, j_pesan);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        
                        if let Some(detach_fn) = (*(*thread_jvm)).DetachCurrentThread {
                            detach_fn(thread_jvm);
                        }
                    }
                }
            }
            // Auto-reconnect jika terputus
            std::thread::sleep(Duration::from_secs(1));
        }
    });
}
