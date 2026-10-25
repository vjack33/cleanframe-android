package com.cleanframe.tv;

import android.app.Activity;
import android.content.Intent;
import android.content.res.AssetManager;
import android.net.Uri;
import android.os.Bundle;
import android.provider.Settings;
import android.view.KeyEvent;
import android.view.View;
import android.view.WindowManager;
import android.webkit.WebResourceRequest;
import android.webkit.WebResourceResponse;
import android.webkit.WebSettings;
import android.webkit.WebView;
import android.webkit.WebViewClient;

import java.io.IOException;
import java.io.InputStream;

public class MainActivity extends Activity {
    private WebView webView;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        android.util.Log.d("CLEANFRAME_TV", "onCreate: Starting CleanFrame MainActivity");

        // Keep screen on and immerse in fullscreen TV mode
        getWindow().addFlags(WindowManager.LayoutParams.FLAG_KEEP_SCREEN_ON);
        hideSystemUI();

        webView = new WebView(this);
        setContentView(webView);

        WebSettings settings = webView.getSettings();
        settings.setJavaScriptEnabled(true);
        settings.setDomStorageEnabled(true);
        settings.setDatabaseEnabled(true);
        settings.setAllowFileAccess(true);
        settings.setAllowContentAccess(true);
        settings.setAllowFileAccessFromFileURLs(true);
        settings.setAllowUniversalAccessFromFileURLs(true);
        settings.setMediaPlaybackRequiresUserGesture(false);
        settings.setSupportZoom(false);
        settings.setBuiltInZoomControls(false);
        settings.setUseWideViewPort(true);
        settings.setLoadWithOverviewMode(true);

        // Enable WebView debugging and console logging
        WebView.setWebContentsDebuggingEnabled(true);
        webView.setWebChromeClient(new android.webkit.WebChromeClient() {
            @Override
            public boolean onConsoleMessage(android.webkit.ConsoleMessage cm) {
                android.util.Log.d("CLEANFRAME_CONSOLE", cm.message() + " [" + cm.sourceId() + ":" + cm.lineNumber() + "]");
                return true;
            }
        });

        // Register the JavaScript interface
        webView.addJavascriptInterface(new WebAppInterface(this), "AndroidBridge");

        // Custom WebViewClient to serve local assets seamlessly
        webView.setWebViewClient(new WebViewClient() {
            @Override
            public void onPageFinished(WebView view, String url) {
                super.onPageFinished(view, url);
                android.util.Log.d("CLEANFRAME_TV", "onPageFinished: " + url);
                view.evaluateJavascript(
                    "JSON.stringify({ qa: (window.QA && window.QA.where ? window.QA.where() : 'no-qa'), " +
                    "dockClass: (document.getElementById('dock') ? document.getElementById('dock').className : 'none'), " +
                    "appsLen: (window.CleanFrameApps ? window.CleanFrameApps.length : 'none'), " +
                    "stageFocus: (document.activeElement ? document.activeElement.tagName + '#' + document.activeElement.id : 'none') })",
                    value -> {
                        android.util.Log.d("CLEANFRAME_DOM", "STATE: " + value);
                        view.evaluateJavascript(
                            "JSON.stringify({ innerW: window.innerWidth, innerH: window.innerHeight, " +
                            "stageRect: document.getElementById('stage').getBoundingClientRect(), " +
                            "dockRect: document.getElementById('dock').getBoundingClientRect(), " +
                            "dockOpacity: window.getComputedStyle(document.getElementById('dock')).opacity, " +
                            "dockDisplay: window.getComputedStyle(document.getElementById('dock')).display, " +
                            "dockVisibility: window.getComputedStyle(document.getElementById('dock')).visibility })",
                            val -> android.util.Log.d("CLEANFRAME_GEOM", "GEOM: " + val)
                        );
                    }
                );
            }

            @Override
            public void onReceivedError(WebView view, int errorCode, String description, String failingUrl) {
                super.onReceivedError(view, errorCode, description, failingUrl);
                android.util.Log.e("CLEANFRAME_TV", "onReceivedError: " + description + " for " + failingUrl);
            }

            @Override
            public WebResourceResponse shouldInterceptRequest(WebView view, WebResourceRequest request) {
                Uri uri = request.getUrl();
                String path = uri.getPath();
                if (path != null) {
                    if (path.startsWith("/")) path = path.substring(1);
                    if (path.isEmpty() || path.equals("home") || path.equals("home/")) path = "index.html";
                    try {
                        InputStream stream = getAssets().open(path);
                        String mimeType = getMimeType(path);
                        return new WebResourceResponse(mimeType, "UTF-8", stream);
                    } catch (IOException ignored) {}
                }
                return super.shouldInterceptRequest(view, request);
            }
        });

        webView.setFocusable(true);
        webView.setFocusableInTouchMode(true);
        webView.requestFocus();

        webView.loadUrl("file:///android_asset/index.html");
    }

    private void hideSystemUI() {
        View decorView = getWindow().getDecorView();
        decorView.setSystemUiVisibility(
                View.SYSTEM_UI_FLAG_IMMERSIVE_STICKY
                | View.SYSTEM_UI_FLAG_LAYOUT_STABLE
                | View.SYSTEM_UI_FLAG_LAYOUT_HIDE_NAVIGATION
                | View.SYSTEM_UI_FLAG_LAYOUT_FULLSCREEN
                | View.SYSTEM_UI_FLAG_HIDE_NAVIGATION
                | View.SYSTEM_UI_FLAG_FULLSCREEN
        );
    }

    @Override
    public void onWindowFocusChanged(boolean hasFocus) {
        super.onWindowFocusChanged(hasFocus);
        if (hasFocus) {
            hideSystemUI();
            if (webView != null) {
                webView.requestFocus();
            }
        }
    }

    @Override
    protected void onNewIntent(android.content.Intent intent) {
        super.onNewIntent(intent);
        if (intent != null && intent.hasExtra("eval")) {
            String script = intent.getStringExtra("eval");
            if (webView != null) {
                webView.evaluateJavascript(script, res -> android.util.Log.d("CLEANFRAME_EVAL", "EVAL: " + res));
            }
        }
    }

    public WebView getWebView() {
        return webView;
    }

    public void openTvSettings() {
        Intent intent = getPackageManager().getLeanbackLaunchIntentForPackage("com.android.tv.settings");
        if (intent == null) intent = new Intent(Settings.ACTION_SETTINGS);
        intent.addFlags(Intent.FLAG_ACTIVITY_NEW_TASK);
        startActivity(intent);
    }

    @Override
    public boolean dispatchKeyEvent(KeyEvent event) {
        int keyCode = event.getKeyCode();
        String keyName = null;
        int jsKeyCode = 0;

        if (keyCode == KeyEvent.KEYCODE_SETTINGS) {
            if (event.getAction() == KeyEvent.ACTION_DOWN) openTvSettings();
            return true;
        }

        switch (keyCode) {
            case KeyEvent.KEYCODE_DPAD_LEFT:
                keyName = "ArrowLeft";
                jsKeyCode = 37;
                break;
            case KeyEvent.KEYCODE_DPAD_UP:
                keyName = "ArrowUp";
                jsKeyCode = 38;
                break;
            case KeyEvent.KEYCODE_DPAD_RIGHT:
                keyName = "ArrowRight";
                jsKeyCode = 39;
                break;
            case KeyEvent.KEYCODE_DPAD_DOWN:
                keyName = "ArrowDown";
                jsKeyCode = 40;
                break;
            case KeyEvent.KEYCODE_DPAD_CENTER:
            case KeyEvent.KEYCODE_ENTER:
            case KeyEvent.KEYCODE_NUMPAD_ENTER:
                keyName = "Enter";
                jsKeyCode = 13;
                break;
            case KeyEvent.KEYCODE_BACK:
            case KeyEvent.KEYCODE_ESCAPE:
                keyName = "Escape";
                jsKeyCode = 27;
                break;
        }

        if (keyName != null && webView != null) {
            String eventType = (event.getAction() == KeyEvent.ACTION_DOWN) ? "keydown" : "keyup";
            String script = "document.dispatchEvent(new KeyboardEvent('" + eventType + "', { "
                    + "key: '" + keyName + "', "
                    + "code: '" + keyName + "', "
                    + "keyCode: " + jsKeyCode + ", "
                    + "which: " + jsKeyCode + ", "
                    + "bubbles: true, cancelable: true }));";
            webView.evaluateJavascript(script, null);
            return true;
        }
        return super.dispatchKeyEvent(event);
    }

    private String getMimeType(String path) {
        if (path.endsWith(".html")) return "text/html";
        if (path.endsWith(".css")) return "text/css";
        if (path.endsWith(".js")) return "application/javascript";
        if (path.endsWith(".svg")) return "image/svg+xml";
        if (path.endsWith(".jpg") || path.endsWith(".jpeg")) return "image/jpeg";
        if (path.endsWith(".png")) return "image/png";
        if (path.endsWith(".woff2")) return "font/woff2";
        if (path.endsWith(".woff")) return "font/woff";
        if (path.endsWith(".ttf")) return "font/ttf";
        if (path.endsWith(".json")) return "application/json";
        return "application/octet-stream";
    }
}
