use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;
use std::fs;

// Fungsi diagnosis GPU yang aman dari resiko Panic/Crash
fn test_gpu_allocation() -> String {
    let mut report = String::new();
    report.push_str("--- SAFE GPU HARDWARE REPORT ---\n");
    
    // Uji coba membaca direktori driver secara aman tanpa memicu crash jika ditolak
    match fs::metadata("/dev/mali0") {
        Ok(_) => report.push_str("[OK] Driver /dev/mali0 terdeteksi dan dapat diakses!\n"),
        Err(e) => report.push_str(&format!("[BLOCKED] /dev/mali0 gagal diakses: {}\n", e)),
    }

    match fs::metadata("/dev/ion") {
        Ok(_) => report.push_str("[OK] Shared Memory /dev/ion aktif!\n"),
        Err(e) => report.push_str(&format!("[INFO] /dev/ion detail: {}\n", e)),
    }

    // Solusi bypass Eror -1000: Informasikan bahwa inisialisasi harus via OpenGL Context Java
    report.push_str("\n[ANALISIS -1000]: Untuk bypass murni di Android 10, alokasi OpenCL wajib dipancing lewat EGLContext di sisi Java MainActivity terlebih dahulu sebelum dilempar ke Rust.");
    report
}

#[no_mangle]
pub extern "C" fn Java_com_godico_devhub_MainActivity_startIPCServer() {
    thread::spawn(|| {
        // Tambahkan catch_unwind atau gunakan unwrap secara hati-hati
        let listener = TcpListener::bind("0.0.0.0:8080");
        if listener.is_err() { return; }
        let listener = listener.unwrap();

        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let mut buffer = [0; 512];
                    if let Ok(bytes_read) = stream.read(&mut buffer) {
                        let request = String::from_utf8_lossy(&buffer[..bytes_read]);
                        
                        if request == "STUDIO:VIDEO" {
                            let response = "STATUS:JNI_ALIVE";
                            let _ = stream.write_all(response.as_bytes());
                        } 
                        else if request == "ENGINE:GPU_TEST" {
                            let response = test_gpu_allocation();
                            let _ = stream.write_all(response.as_bytes());
                        } else {
                            let _ = stream.write_all(b"STATUS:UNKNOWN_COMMAND");
                        }
                    }
                }
                Err(_) => {}
            }
        }
    });
}
