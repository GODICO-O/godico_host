package com.godico.devhub;

import android.graphics.Color;
import android.os.Bundle;
import android.widget.Button;
import android.widget.TextView;
import androidx.appcompat.app.AppCompatActivity;

public class MainActivity extends AppCompatActivity {
    
    // Kita hapus static loadLibrary biar gak berat di awal!
    public native boolean checkRootStatus();

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);

        Button verify_root = findViewById(R.id.verify_root);
        TextView root_status = findViewById(R.id.root_status);

        verify_root.setOnClickListener(v -> {
            new Thread(() -> {
                // Load library hanya saat tombol ditekan (Lazy Loading)
                try {
                    System.loadLibrary("rust_core");
                } catch (UnsatisfiedLinkError e) {
                    // Berarti library sudah pernah dimuat sebelumnya
                }

                boolean rooted = checkRootStatus();
                
                runOnUiThread(() -> {
                    root_status.setText(rooted ? "Your Phone Is Rooted" : "Phone Is Not Rooted");
                    root_status.setTextColor(Color.parseColor(rooted ? "#00FF00" : "#FF0000"));
                });
            }).start();
        });
    }
}
