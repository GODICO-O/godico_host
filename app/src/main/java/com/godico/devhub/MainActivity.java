package com.godico.devhub;

import android.os.Bundle;
import android.widget.TextView;
import androidx.appcompat.app.AppCompatActivity;

public class MainActivity extends AppCompatActivity {

    private TextView logTextView;

    static {
        // Memuat biner Rust_Core hasil bakaran lokal
        System.loadLibrary("rust_core");
    }

    // Deklarasi fungsi asli JNI Rust
    public native void startIpcServer();

    @Override
    protected void onCreate(Bundle bundle) {
        super.onCreate(bundle);
        
        // Membuat UI minimalis konsol hitam-hijau hacker secara dinamis
        logTextView = new TextView(this);
        logTextView.setTextSize(16);
        logTextView.setBackgroundColor(0xFF000000); // Hitam
        logTextView.setTextColor(0xFF00FF00);       // Hijau Matriks
        logTextView.setPadding(30, 30, 30, 30);
        logTextView.setText("=== GODICO DEVHUB LOG MONITOR ===\n[SYSTEM]: Menginisialisasi Pipa Rust...\n");
        
        setContentView(logTextView);

        // Jalankan server monitor aktif Rust di latar belakang
        startIpcServer();
    }

    // Fungsi pembaruan teks yang WAJIB dipaksa berjalan di UI Thread Utama Android
    public void updateLogText(final String teks) {
        runOnUiThread(new Runnable() {
            @Override
            public void run() {
                if (logTextView != null) {
                    // Tambahkan teks baru di bawah baris sebelumnya (Append)
                    logTextView.append("\n" + teks);
                }
            }
        });
    }
}
