use jni_sys::{JNIEnv, jobject};
use ndk_sys::{ANativeWindow_fromSurface, ANativeWindow_lock, ANativeWindow_unlockAndPost, ANativeWindow_Buffer};

#[no_mangle]
pub unsafe extern "C" fn Java_com_godico_devhub_MainActivity_initRenderer(env: *mut JNIEnv, _: jobject, surface: jobject) {
    // Karena env sudah bertipe *mut JNIEnv, kita bisa langsung pasang!
    let window = ANativeWindow_fromSurface(env as *mut _, surface);
    
    if window.is_null() { return; }

    for i in 0..100 {
        let mut buffer: ANativeWindow_Buffer = std::mem::zeroed();
        if ANativeWindow_lock(window, &mut buffer, std::ptr::null_mut()) == 0 {
            let pixels = buffer.bits as *mut u32;
            let color = if i % 2 == 0 { 0xFFFF0000 } else { 0xFF00FF00 }; // ARGB: Alpha-Red-Green-Blue
            
            for y in 0..buffer.height {
                for x in 0..buffer.width {
                    *pixels.add(y as usize * buffer.stride as usize + x as usize) = color;
                }
            }
            ANativeWindow_unlockAndPost(window);
        }
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
