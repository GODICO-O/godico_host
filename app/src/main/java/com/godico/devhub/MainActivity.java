package com.godico.devhub;

import android.app.Activity;
import android.os.Bundle;
import android.os.Handler;
import android.os.Looper;
import android.widget.TextView;
import android.util.Log;

public class MainActivity extends Activity {
    private TextView logTextView;
    private Handler handler = new Handler(Looper.getMainLooper());
    private String latestData = "[STATUS]: Menunggu data dari Termux...";

    static { System.loadLibrary("rust_core"); }
    public native void startIpcServer();

    @Override
    protected void onCreate(Bundle bundle) {
        super.onCreate(bundle);
        logTextView = new TextView(this);
        logTextView.setBackgroundColor(0xFF000000);
        logTextView.setTextColor(0xFF00FF00);
        logTextView.setPadding(30, 30, 30, 30);
        setContentView(logTextView);

        new Thread(this::startIpcServer).start();
        startHeartbeat();
    }

    private void startHeartbeat() {
        handler.postDelayed(new Runnable() {
            @Override
            public void run() {
                logTextView.setText("=== GODICO MONITOR ===\n" + latestData + "\n\n[HEARTBEAT]: " + System.currentTimeMillis() % 10000);
                handler.postDelayed(this, 2000);
            }
        }, 2000);
    }

    public void updateLogText(final String teks) {
        latestData = "[DATA]: " + teks;
        Log.d("GODICO_DATA", "Data diterima: " + teks);
    }
}
