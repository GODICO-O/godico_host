use std::net::TcpStream;
use std::io::{BufReader, BufRead};
use std::ffi::CString;
use std::time::Duration;

// Menyisipkan fungsi logging bawaan sistem operasi Android
use android_logger::Config;
use log::{info, error, LevelFilter};

type JNIEnvPtr = *mut jni_sys::JNIEnv;
type JobjectPtr = jni_sys::jobject;
type JavaVMPtr = *mut jni_sys::JavaVM;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_godico_devhub_MainActivity_startIpcServer(
    env: JNIEnvPtr,
    jclass: JobjectPtr,
) {
    // Inisialisasi sistem penyadapan log Android (Hanya sekali seumur hidup aplikasi)
    let _ = android_logger::init_once(
        Config::default()
            .with_max_level(LevelFilter::Debug)
            .with_tag("GODICO_RUST")
    );

    info!("Sistem Pengintai Rust AKTIF! Memulai thread IPC...");

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
            info!("Mencoba menusuk gerbang Termux di 127.0.0.1:8080...");
            
            // Kita kembalikan ke localhost karena kita akan sadap eror aslinya
            match TcpStream::connect("127.0.0.1:8080") {
                Ok(stream) => {
                    info!("🔥 BERHASIL HANDSHAKE! Pipa TCP Terkunci dengan Termux!");
                    oper_ke_java(thread_jvm, thread_jclass, "🟢 [CONNECTED]: Jalur Terkunci!");

                    let mut reader = BufReader::new(stream);
                    let mut baris_teks = String::new();
                    
                    while let Ok(bytes) = reader.read_line(&mut baris_teks) {
                        if bytes == 0 { 
                            info!("Koneksi diputus oleh Server Termux.");
                            break; 
                        } 
                        
                        let pesan = baris_teks.trim().to_string();
                        baris_teks.clear(); 
                        
                        if pesan.is_empty() { continue; }

                        info!("Menangkap data dari Termux: {}", pesan);
                        oper_ke_java(thread_jvm, thread_jclass, &pesan);
                    }
                }
                Err(e) => {
                    // DI SINI KITA AKAN TAHU ALASAN UTAMA MENGAPA PIPANYA MAMPET!
                    error!("❌ GAGAL KONEKSI! Alasan Sistem: {:?}", e);
                    oper_ke_java(thread_jvm, thread_jclass, "🔴 [STUCK]: Mencari Frekuensi...");
                }
            }
            std::thread::sleep(Duration::from_secs(3));
        }
    });
}

fn oper_ke_java(thread_jvm: JavaVMPtr, thread_jclass: JobjectPtr, teks: &str) {
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
                            let c_pesan = CString::new(teks).unwrap();
                            let j_pesan = new_string(local_env, c_pesan.as_ptr());
                            
                            if let Some(call_method) = (*(*local_env)).CallVoidMethod {
                                call_method(local_env, thread_jclass, method_id, j_pesan);
                            }
                        }
                    }
                }
            }
        }
        if let Some(detach_fn) = (*(*thread_jvm)).AttachCurrentThread {
            let _ = detach_fn(thread_jvm);
        }
    }
}
