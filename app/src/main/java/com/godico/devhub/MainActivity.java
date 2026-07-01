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
            new Thread(() -> {
                boolean rooted = checkRootStatus();
                runOnUiThread(() -> {
                    root_status.setText(rooted ? "Your Phone Is Rooted" : "Phone Is Not Rooted");
                    root_status.setTextColor(Color.parseColor(rooted ? "#00FF00" : "#FF0000"));
                });
            }).start();
        });
    }
}
