use std::net::TcpStream;
use std::io::{BufReader, BufRead};
use std::ffi::CString;
use std::time::Duration;
use android_logger::Config;
use log::{info, LevelFilter};

type JNIEnvPtr = *mut jni_sys::JNIEnv;
type JobjectPtr = jni_sys::jobject;
type JavaVMPtr = *mut jni_sys::JavaVM;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_godico_devhub_MainActivity_startIpcServer(env: JNIEnvPtr, jclass: JobjectPtr) {
    let _ = android_logger::init_once(Config::default().with_max_level(LevelFilter::Info).with_tag("GODICO_RUST"));
    let mut jvm: JavaVMPtr = std::ptr::null_mut();
    unsafe { if let Some(f) = (*(*env)).GetJavaVM { f(env, &mut jvm); } }
    let (jvm_raw, jclass_raw) = (jvm as usize, jclass as usize);

    std::thread::spawn(move || {
        let (thread_jvm, thread_jclass) = (jvm_raw as JavaVMPtr, jclass_raw as JobjectPtr);
        loop {
            // PORT DIGANTI KE 9999
            if let Ok(stream) = TcpStream::connect_timeout(&"127.0.0.1:9999".parse().unwrap(), Duration::from_secs(3)) {
                let mut reader = BufReader::new(stream);
                let mut line = String::new();
                while let Ok(bytes) = reader.read_line(&mut line) {
                    if bytes == 0 { break; }
                    oper_ke_java(thread_jvm, thread_jclass, line.trim());
                    line.clear();
                }
            }
            std::thread::sleep(Duration::from_secs(2));
        }
    });
}

fn oper_ke_java(thread_jvm: JavaVMPtr, thread_jclass: JobjectPtr, teks: &str) {
    unsafe {
        let mut local_env: JNIEnvPtr = std::ptr::null_mut();
        if let Some(attach_fn) = (*(*thread_jvm)).AttachCurrentThread {
            let res = attach_fn(thread_jvm, &mut local_env as *mut JNIEnvPtr as *mut *mut std::ffi::c_void, std::ptr::null_mut());
            if res == 0 && !local_env.is_null() {
                if let Some(get_method_id) = (*(*local_env)).GetMethodID {
                    let class_target = if let Some(get_class) = (*(*local_env)).GetObjectClass { get_class(local_env, thread_jclass) } else { std::ptr::null_mut() };
                    let method_id = get_method_id(local_env, class_target, CString::new("updateLogText").unwrap().as_ptr(), CString::new("(Ljava/lang/String;)V").unwrap().as_ptr());
                    if !method_id.is_null() {
                        if let Some(new_string) = (*(*local_env)).NewStringUTF {
                            let j_pesan = new_string(local_env, CString::new(teks).unwrap().as_ptr());
                            if let Some(call_method) = (*(*local_env)).CallVoidMethod { call_method(local_env, thread_jclass, method_id, j_pesan); }
                        }
                    }
                }
            }
            let detach_ptr: extern "C" fn(JavaVMPtr) -> i32 = std::mem::transmute((*(*thread_jvm)).DetachCurrentThread);
            let _ = detach_ptr(thread_jvm);
        }
    }
}
