use std::net::TcpListener;
use std::io::{Read, Write};

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_godico_devhub_MainActivity_startIpcServer(
    _env: *mut std::ffi::c_void, _jclass: *mut std::ffi::c_void,
) {
    std::thread::spawn(|| {
        if let Ok(listener) = TcpListener::bind("0.0.0.0:8080") {
            for stream in listener.incoming() {
                if let Ok(mut s) = stream {
                    let mut buffer = [0; 1024];
                    if let Ok(bytes) = s.read(&mut buffer) {
                        let pesan = String::from_utf8_lossy(&buffer[..bytes]);
                        if pesan.starts_with("PIXEL_DAT:") {
                            // Data gambar akan masuk ke sini
                            let _ = s.write_all(b"ACK:PIXELS_RECEIVED");
                        } else {
                            let _ = s.write_all(b"STATUS:IPC_ALIVE");
                        }
                    }
                }
            }
        }
    });
}
