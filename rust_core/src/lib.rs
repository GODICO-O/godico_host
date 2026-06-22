use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;

#[no_mangle]
pub extern "C" fn Java_com_godico_devhub_MainActivity_startIPCServer() {
    thread::spawn(|| {
        let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
        println!("IPC Server berjalan di port 8080...");

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
                    }
                }
                Err(_) => {}
            }
        }
    });
}
