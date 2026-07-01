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
        
        // Buat layout manual biar gak butuh XML layout kalau mau super enteng
        LinearLayout layout = new LinearLayout(this);
        layout.setOrientation(LinearLayout.VERTICAL);
        layout.setGravity(android.view.Gravity.CENTER);
        
        TextView root_status = new TextView(this);
        root_status.setText("Status Root Belum Dicek");
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
                    root_status.setText(rooted ? "Your Phone Is Rooted" : "Phone Is Not Rooted");
                    root_status.setTextColor(Color.parseColor(rooted ? "#00FF00" : "#FF0000"));
                });
            }).start();
        });
    }
}
