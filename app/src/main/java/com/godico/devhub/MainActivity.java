package com.godico.devhub;

import android.app.Activity;
import android.os.Bundle;
import android.widget.TextView;

public class MainActivity extends Activity {
    
    // Panggil biner .so hasil kompilasi robot GitHub Actions
    static {
        System.loadLibrary("rust_core");
    }

    // Deklarasikan fungsi native yang ada di dalam Rust lib.rs
    public native void startIpcServer();

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        
        TextView tv = new TextView(this);
        tv.setText("🗿 GODICO DevHub: ENGINE AKTIF!\nIPC Server berjalan di port 8080 via GitHub Actions...");
        tv.setTextSize(18);
        setContentView(tv);

        // Picu server IPC Rust di latar belakang thread
        startIpcServer();
    }
}
