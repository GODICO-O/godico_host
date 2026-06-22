use std::net::TcpStream;
use std::io::Read;
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
            // SOLUSI MUTLAK: Tembak langsung ke IP WLAN lokal agar menembus sandbox Android!
            // GANTI 192.168.0.110 di bawah ini dengan IP asli HP lu jika berbeda!
            if let Ok(mut stream) = TcpStream::connect("192.168.0.110:8080") {
                let mut buffer = [0; 1024];
                
                while let Ok(bytes_read) = stream.read(&mut buffer) {
                    if bytes_read == 0 { break; } 
                    
                    let pesan_mentah = String::from_utf8_lossy(&buffer[..bytes_read]);
                    let pesan = pesan_mentah.trim().to_string();
                    
                    if pesan.is_empty() { continue; }

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
            std::thread::sleep(Duration::from_secs(1));
        }
    });
}
