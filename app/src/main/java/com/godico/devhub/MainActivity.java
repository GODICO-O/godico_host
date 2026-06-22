package com.godico.devhub;

import android.app.Activity;
import android.os.Bundle;
import android.widget.TextView;
import java.text.SimpleDateFormat;
import java.util.Date;
import java.util.Locale;

public class MainActivity extends Activity {

    private TextView tvLog;

    static {
        System.loadLibrary("rust_core");
    }

    public native void startIpcServer();

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);

        // Buat tampilan monitor log yang rapi dan futuristik
        tvLog = new TextView(this);
        tvLog.setText("🗿 GODICO LIVE LOG MONITOR 🗿\n\n[Sistem] Menunggu kiriman log dari Termux...\n-----------------------------------------");
        tvLog.setTextSize(16);
        tvLog.setPadding(50, 50, 50, 50);
        tvLog.setBackgroundColor(0xFF1E1E1E); // Latar belakang hitam konsol
        tvLog.setTextColor(0xFF00FF00);       // Teks warna hijau matriks
        setContentView(tvLog);

        // Jalankan server IPC Rust sambil mengoper instance objek saat ini
        startIpcServer();
    }

    // FUNGSI SAKTI: Akan dipanggil langsung oleh Rust di latar belakang
    public void updateLogText(final String logBaru) {
        // Wajib dilempar ke runOnUiThread agar sistem Android tidak crash saat memanipulasi teks
        runOnUiThread(new Runnable() {
            @Override
            public void run() {
                String timeStamp = new SimpleDateFormat("HH:mm:ss", Locale.getDefault()).format(new Date());
                String teksLama = tvLog.getText().toString();
                
                // Tambahkan log baru di baris paling bawah
                String teksBaru = teksLama + "\n[" + timeStamp + "] " + logBaru;
                tvLog.setText(teksBaru);
            }
        });
    }
}
