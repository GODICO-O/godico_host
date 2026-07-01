package com.godico.devhub;

import android.app.Activity;
import android.os.Bundle;
import android.widget.Button;
import android.widget.TextView;
import android.graphics.Color;
import android.view.ViewGroup;
import android.widget.LinearLayout;

public class MainActivity extends Activity {

    public native boolean checkRootStatus();

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);

        // Mengatur orientasi agar mengikuti sensor (bisa portrait & landscape)
        setRequestedOrientation(5); 

        // Buat layout manual biar gak butuh XML layout kalau mau super enteng
        LinearLayout layout = new LinearLayout(this);
        layout.setOrientation(LinearLayout.VERTICAL);
        layout.setGravity(android.view.Gravity.CENTER);

        TextView root_status = new TextView(this);
        root_status.setText("Tekan Tombol Untuk Cek");
        root_status.setTextSize(20f);

        Button verify_root = new Button(this);
        verify_root.setText("Verify Root");

        layout.addView(root_status);
        layout.addView(verify_root);
        setContentView(layout);

        verify_root.setOnClickListener(v -> {
            new Thread(() -> {
                try {
                    System.loadLibrary("rust_core");
                } catch (UnsatisfiedLinkError e) {}

                boolean rooted = checkRootStatus();

                runOnUiThread(() -> {
                    // Memperbaiki logika teks dan warna berdasarkan status root
                    if (rooted) {
                        root_status.setText("Your Phone Is Rooted");
                        root_status.setTextColor(Color.parseColor("#FF0000")); // Merah jika rooted
                    } else {
                        root_status.setText("Your Phone Is Not Rooted");
                        root_status.setTextColor(Color.parseColor("#00FF00")); // Hijau jika aman
                    }
                });
            }).start();
        });
    }
}
