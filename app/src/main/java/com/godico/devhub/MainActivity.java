package com.godico.devhub;

import android.graphics.Color;
import android.os.Bundle;
import android.widget.Button;
import android.widget.TextView;
import androidx.appcompat.app.AppCompatActivity;

public class MainActivity extends AppCompatActivity {

    static {
        System.loadLibrary("rust_core");
    }

    public native boolean checkRootStatus();

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);

        Button verify_root = findViewById(R.id.verify_root);
        TextView root_status = findViewById(R.id.root_status);

        verify_root.setOnClickListener(v -> {
            // Panggil di background thread biar UI gak freeze
            new Thread(() -> {
                boolean rooted = checkRootStatus();
                
                runOnUiThread(() -> {
                    if (rooted) {
                        root_status.setText("Your Phone Is Rooted");
                        root_status.setTextColor(Color.parseColor("#00FF00"));
                    } else {
                        root_status.setText("Phone Is Not Rooted");
                        root_status.setTextColor(Color.parseColor("#FF0000"));
                    }
                });
            }).start();
        });
    }
}
