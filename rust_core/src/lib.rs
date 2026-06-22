use std::net::TcpListener;
use std::io::{Read, Write};
use std::ffi::CString;

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

        // Buka gerbang pelabuhan port 8080
        match TcpListener::bind("0.0.0.0:8080") {
            Ok(listener) => {
                for stream in listener.incoming() {
                    match stream {
                        Ok(mut s) => {
                            // SOLUSI 1: Perbesar buffer menjadi 1024 bytes agar teks panjang tidak buntung
                            let mut buffer = [0; 1024]; 
                            if let Ok(bytes_read) = s.read(&mut buffer) {
                                if bytes_read == 0 { continue; }
                                
                                let pesan = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();
                                
                                // Kirim balasan aman ke Termux
                                let respon = format!("STATUS:LOG_DISPLAYED | Konten: {}", pesan);
                                let _ = s.write_all(respon.as_bytes());

                                // EKSEKUSI JNI DENGAN PENGAMAN MUTLAK
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
                                                        let c_pesan = CString::new(pesan).unwrap();
                                                        let j_pesan = new_string(local_env, c_pesan.as_ptr());
                                                        
                                                        if let Some(call_method) = (*(*local_env)).CallVoidMethod {
                                                            call_method(local_env, thread_jclass, method_id, j_pesan);
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    
                                    // SOLUSI 2: WAJIB DETACH THREAD AGAR ANDROID TIDAK LEAK / CRASH AUTO-REFRESH!
                                    if let Some(detach_fn) = (*(*thread_jvm)).DetachCurrentThread {
                                        detach_fn(thread_jvm);
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
