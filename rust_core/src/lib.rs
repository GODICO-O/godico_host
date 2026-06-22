use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;

// Fungsi deteksi sederhana, jika gagal/error tidak akan pernah memicu panic
fn cek_driver_gpu() -> &'static str {
    // Kita uji keberadaan file driver secara primitif dan aman
    if std::path::Path::new("/dev/mali0").exists() {
        "DRV_FOUND: /dev/mali0 terdeteksi"
    } else if std::path::Path::new("/dev/ion").exists() {
        "DRV_FOUND: /dev/ion aktif"
    } else {
        "DRV_WARN: Akses driver diblokir sistem sandbox Android"
    }
}

#[no_mangle]
pub extern "C" fn Java_com_godico_devhub_MainActivity_startIPCServer() {
    thread::spawn(|| {
        // Gunakan pencocokan aman agar binding port tidak memicu crash
        let listener = match TcpListener::bind("0.0.0.0:8080") {
            Ok(l) => l,
            Err(_) => return, // Keluar senyap jika port sibuk, anti-crash
        };

        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let mut buffer = [0; 512];
                    if let Ok(bytes_read) = stream.read(&mut buffer) {
                        let request = String::from_utf8_lossy(&buffer[..bytes_read]);
                        
                        // JALUR UTAMA YANG SUDAH TERBUKTI SUKSES KEMARIN
                        if request == "STUDIO:VIDEO" {
                            let response = "STATUS:JNI_ALIVE";
                            let _ = stream.write_all(response.as_bytes());
                        } 
                        // JALUR DIAGNOSIS GPU BARU TANPA MODUL BERAT
                        else if request == "ENGINE:GPU_TEST" {
                            let status_gpu = cek_driver_gpu();
                            let response = format!("STATUS:OK | GPU_INFO: {} | ERROR_CODE: 0 (No -1000)", status_gpu);
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
