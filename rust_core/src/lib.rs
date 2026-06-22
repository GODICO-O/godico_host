use std::net::TcpListener;
use std::io::{Read, Write};

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_godico_devhub_MainActivity_startIpcServer(
    _env: *mut std::ffi::c_void,
    _jclass: *mut std::ffi::c_void,
) {
    // Jalankan server di dalam thread terpisah agar runtime Java Android tetap bernapas lega
    std::thread::spawn(|| {
        match TcpListener::bind("0.0.0.0:8080") {
            Ok(listener) => {
                for stream in listener.incoming() {
                    match stream {
                        Ok(mut s) => {
                            let mut buffer = [0; 128];
                            if let Ok(bytes_read) = s.read(&mut buffer) {
                                let pesan = String::from_utf8_lossy(&buffer[..bytes_read]);
                                if pesan.contains("STUDIO:VIDEO") {
                                    // Sinyal balik sementara untuk membuktikan JNI dan Port 8080 Hidup Sehat!
                                    s.write_all(b"STATUS:JNI_ALIVE").unwrap();
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
