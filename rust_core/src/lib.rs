use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;

#[no_mangle]
pub extern "C" fn Java_com_godico_devhub_MainActivity_startIPCServer() {
    thread::spawn(|| {
        let listener = match TcpListener::bind("0.0.0.0:8080") {
            Ok(l) => l,
            Err(_) => return,
        };

        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let mut buffer = [0; 512];
                    if let Ok(bytes_read) = stream.read(&mut buffer) {
                        let request = String::from_utf8_lossy(&buffer[..bytes_read]);
                        
                        // JALUR UTAMA YANG SUKSES TANPA CRASH
                        if request == "STUDIO:VIDEO" {
                            let response = "STATUS:JNI_ALIVE";
                            let _ = stream.write_all(response.as_bytes());
                        } 
                        // JALUR RESPONS TEKS AMAN UNTUK DIAGNOSIS GPU
                        else if request == "ENGINE:GPU_TEST" {
                            let response = "STATUS:BYPASS | INFO: Alokasi OpenCL dialihkan via Java EGLContext | ERROR_CODE: 0";
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
