package com.godico.devhub;

import android.app.Activity;
import android.os.Bundle;
import android.widget.TextView;
import android.view.SurfaceView;
import android.view.ViewGroup;

public class MainActivity extends Activity {
    static { System.loadLibrary("rust_core"); }
    public native void startIpcServer();

    @Override
    protected void onCreate(Bundle bundle) {
        super.onCreate(bundle);
        
        // Buat container untuk teks dan kanvas
        TextView tv = new TextView(this);
        tv.setText("🗿 VOID-Display: ONLINE\nMenunggu Aliran Pixel...");
        
        SurfaceView surface = new SurfaceView(this);
        surface.setLayoutParams(new ViewGroup.LayoutParams(500, 500));
        
        setContentView(tv);
        startIpcServer();
    }
}
