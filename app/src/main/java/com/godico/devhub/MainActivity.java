package com.godico.devhub;

import android.app.Activity;
import android.os.Bundle;
import android.view.Surface;
import android.view.SurfaceHolder;
import android.view.SurfaceView;

public class MainActivity extends Activity implements SurfaceHolder.Callback {
    static { System.loadLibrary("rust_core"); }
    public native void initRenderer(Surface surface);

    @Override
    protected void onCreate(Bundle bundle) {
        super.onCreate(bundle);
        SurfaceView surfaceView = new SurfaceView(this);
        surfaceView.getHolder().addCallback(this);
        setContentView(surfaceView);
    }

    @Override public void surfaceCreated(SurfaceHolder holder) { initRenderer(holder.getSurface()); }
    @Override public void surfaceChanged(SurfaceHolder h, int f, int w, int hgt) {}
    @Override public void surfaceDestroyed(SurfaceHolder h) {}
}
