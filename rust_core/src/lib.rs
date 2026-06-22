use jni_sys::{JNIEnv, jobject};
use ndk_sys::{ANativeWindow_fromSurface, ANativeWindow_lock, ANativeWindow_unlockAndPost, ANativeWindow_Buffer, ANativeWindow_acquire, ANativeWindow_release};

#[no_mangle]
pub unsafe extern "C" fn Java_com_godico_devhub_MainActivity_initRenderer(env: *mut JNIEnv, _: jobject, surface: jobject) {
    let window = ANativeWindow_fromSurface(env as *mut _, surface);
    if window.is_null() { return; }
    
    // Acquire window agar tidak direbut oleh sistem saat kita sedang menggambar
    ANativeWindow_acquire(window);

    for i in 0..100 {
        let mut buffer: ANativeWindow_Buffer = std::mem::zeroed();
        // Coba kunci buffer dengan mencoba beberapa kali
        if ANativeWindow_lock(window, &mut buffer, std::ptr::null_mut()) == 0 {
            let pixels = buffer.bits as *mut u32;
            let color = if i % 2 == 0 { 0xFFFF0000 } else { 0xFF00FF00 };
            
            for y in 0..buffer.height {
                let row = y as usize * buffer.stride as usize;
                for x in 0..buffer.width {
                    *pixels.add(row + x as usize) = color;
                }
            }
            ANativeWindow_unlockAndPost(window);
        }
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
    
    ANativeWindow_release(window);
}
