package com.cleanframe.tv;

import android.content.Context;
import android.content.Intent;
import android.content.pm.PackageManager;
import android.content.pm.ResolveInfo;
import android.media.tv.TvInputInfo;
import android.media.tv.TvInputManager;
import android.net.Uri;
import android.os.Build;
import android.provider.Settings;
import android.webkit.JavascriptInterface;
import org.json.JSONArray;
import org.json.JSONObject;

import java.util.HashMap;
import java.util.HashSet;
import java.util.List;
import java.util.Map;
import java.util.Set;

public class WebAppInterface {
    private final MainActivity activity;
    private static final Map<String, String> BRAND_PACKAGES = new HashMap<>();
    private static final Map<String, String> PACKAGE_BRANDS = new HashMap<>();

    static {
        addBrand("netflix", "com.netflix.ninja");
        addBrand("youtube", "com.google.android.youtube.tv");
        addBrand("primevideo", "com.amazon.amazonvideo.livingroom");
        addBrand("disneyplus", "com.disney.disneyplus");
        addBrand("spotify", "com.spotify.tv.android");
        addBrand("stremio", "com.stremio.one");
        addBrand("appletv", "com.apple.atve.androidtv.appletv");
        addBrand("applemusic", "com.apple.android.music");
        addBrand("hbomax", "com.wbd.stream");
        addBrand("tubi", "com.tubitv");
        addBrand("abciview", "au.net.abc.iview");
        addBrand("sbs", "sbs.ondemand.tv");
        addBrand("zee5", "com.graymatrix.did");
        addBrand("plex", "com.plexapp.android");
        addBrand("jellyfin", "org.jellyfin.androidtv");
        addBrand("wholphin", "com.github.damontecres.wholphin");
        addBrand("aptoide", "cm.aptoidetv.pt");
        addBrand("aurora", "com.aurora.store");
        addBrand("projectivy", "com.spocky.projengmenu");
        addBrand("tvsettings", "com.android.tv.settings");
        addBrand("mediaplayer", "com.tcl.ui_mediaCenter");
        addBrand("livetv", "com.tcl.tv");
        addBrand("tclhome", "com.tcl.dashboard");
        addBrand("quickpanel", "com.tcl.suspension");

        // Also support alternate common package names
        PACKAGE_BRANDS.put("com.premium.tv.launcher.ui", "projectivy");
        PACKAGE_BRANDS.put("org.smarttube.stable", "youtube");
        PACKAGE_BRANDS.put("org.smarttube.beta", "youtube");
        PACKAGE_BRANDS.put("in.startv.hotstar", "disneyplus");
        PACKAGE_BRANDS.put("com.liskovsoft.videomanager", "youtube");
        PACKAGE_BRANDS.put("com.teamsmart.videomanager.tv", "youtube");
        PACKAGE_BRANDS.put("com.hbo.hbonow", "hbomax");
    }

    private static void addBrand(String brand, String pkg) {
        BRAND_PACKAGES.put(brand, pkg);
        PACKAGE_BRANDS.put(pkg, brand);
    }

    private boolean isInstalled(String packageName) {
        try {
            activity.getPackageManager().getPackageInfo(packageName, 0);
            return true;
        } catch (Exception e) {
            return false;
        }
    }

    public WebAppInterface(MainActivity activity) {
        this.activity = activity;
    }

    @JavascriptInterface
    public String launchApp(String packageName) {
        try {
            if (packageName == null || packageName.isEmpty()) {
                return "Empty package name";
            }
            String resolved = BRAND_PACKAGES.get(packageName);
            if (resolved == null) resolved = packageName;

            // Resolve brand aliases & alternates if target package is not installed directly
            if ("youtube".equals(packageName) || "com.google.android.youtube.tv".equals(resolved)) {
                if (!isInstalled(resolved) && isInstalled("org.smarttube.stable")) resolved = "org.smarttube.stable";
            }
            if ("disneyplus".equals(packageName) || "com.disney.disneyplus".equals(resolved)) {
                if (!isInstalled(resolved) && isInstalled("in.startv.hotstar")) resolved = "in.startv.hotstar";
            }
            if ("appletv".equals(packageName) || packageName.contains("aria-video") || packageName.contains("appletv")) {
                resolved = "com.apple.atve.androidtv.appletv";
            }
            if ("primevideo".equals(packageName) || packageName.contains("primevideo")) {
                resolved = "com.amazon.amazonvideo.livingroom";
            }
            if ("spotify".equals(packageName)) {
                resolved = "com.spotify.tv.android";
            }
            if ("zee5".equals(packageName) || packageName.contains("zee5") || packageName.contains("graymatrix")) {
                resolved = "com.graymatrix.did";
            }
            if ("plex".equals(packageName) || packageName.contains("plexapp")) {
                resolved = "com.plexapp.android";
            }
            if ("jellyfin".equals(packageName) || packageName.contains("jellyfin")) {
                resolved = "org.jellyfin.androidtv";
            }
            if ("wholphin".equals(packageName) || packageName.contains("wholphin")) {
                resolved = "com.github.damontecres.wholphin";
            }
            if ("aptoide".equals(packageName) || packageName.contains("aptoidetv")) {
                resolved = "cm.aptoidetv.pt";
            }
            if ("aurora".equals(packageName) || packageName.contains("aurora.store")) {
                resolved = "com.aurora.store";
            }
            if ("projectivy".equals(packageName) || packageName.contains("projengmenu") || packageName.contains("premium.tv.launcher")) {
                resolved = isInstalled("com.spocky.projengmenu") ? "com.spocky.projengmenu" : "com.premium.tv.launcher.ui";
            }
            if ("tvsettings".equals(packageName) || "settings".equals(packageName) || packageName.contains("tv.settings")) {
                resolved = "com.android.tv.settings";
            }
            if ("mediaplayer".equals(packageName) || packageName.contains("ui_mediaCenter")) {
                resolved = "com.tcl.ui_mediaCenter";
            }
            if ("livetv".equals(packageName) || "tv".equals(packageName) || packageName.contains("tcl.tv")) {
                resolved = "com.tcl.tv";
            }
            if ("tclhome".equals(packageName) || packageName.contains("tcl.dashboard")) {
                resolved = "com.tcl.dashboard";
            }
            if ("quickpanel".equals(packageName) || packageName.contains("tcl.suspension")) {
                resolved = "com.tcl.suspension";
            }

            PackageManager pm = activity.getPackageManager();
            Intent intent = pm.getLeanbackLaunchIntentForPackage(resolved);
            if (intent == null) {
                intent = pm.getLaunchIntentForPackage(resolved);
            }
            if (intent != null) {
                intent.addFlags(Intent.FLAG_ACTIVITY_NEW_TASK);
                activity.startActivity(intent);
                return "ok";
            }
            return "Application not found: " + resolved;
        } catch (Exception e) {
            return e.getMessage() != null ? e.getMessage() : "Launch error";
        }
    }

    @JavascriptInterface
    public void launchBrand(String brand) {
        launchApp(brand);
    }

    @JavascriptInterface
    public String getTvInputs() {
        JSONArray array = new JSONArray();
        try {
            TvInputManager manager = (TvInputManager) activity.getSystemService(Context.TV_INPUT_SERVICE);
            if (manager == null) return array.toString();
            for (TvInputInfo info : manager.getTvInputList()) {
                if (info == null || !info.isPassthroughInput()) continue;
                String id = info.getId();
                CharSequence sourceLabel = info.loadLabel(activity);
                String label = sourceLabel == null ? id : sourceLabel.toString();
                boolean connected = false;
                try {
                    connected = manager.getInputState(id) == TvInputManager.INPUT_STATE_CONNECTED;
                } catch (Exception ignored) {}
                JSONObject item = new JSONObject();
                item.put("id", id);
                item.put("label", label);
                item.put("connected", connected);
                array.put(item);
            }
        } catch (Exception e) {
            android.util.Log.w("CLEANFRAME_TV", "Could not enumerate TV inputs", e);
        }
        return array.toString();
    }

    @JavascriptInterface
    public String launchInput(String inputId) {
        try {
            if (inputId == null || inputId.isEmpty()) return "Empty input id";
            TvInputManager manager = (TvInputManager) activity.getSystemService(Context.TV_INPUT_SERVICE);
            boolean available = false;
            if (manager != null) {
                for (TvInputInfo info : manager.getTvInputList()) {
                    if (info != null && info.isPassthroughInput() && inputId.equals(info.getId())) {
                        available = true;
                        break;
                    }
                }
            }
            if (!available) return "This TV input is no longer available.";
            Intent intent = new Intent(Intent.ACTION_VIEW);
            intent.setData(Uri.parse("content://android.media.tv/passthrough/" + Uri.encode(inputId)));
            intent.addFlags(Intent.FLAG_ACTIVITY_NEW_TASK);
            if (intent.resolveActivity(activity.getPackageManager()) == null) {
                return "This TV does not support launching HDMI inputs.";
            }
            activity.startActivity(intent);
            return "ok";
        } catch (Exception e) {
            return "This TV does not support launching HDMI inputs.";
        }
    }

    @JavascriptInterface
    public String getNowPlaying() {
        return MediaNotificationListener.snapshot(activity);
    }

    @JavascriptInterface
    public String controlMedia(String action) {
        return MediaNotificationListener.control(activity, action);
    }

    @JavascriptInterface
    public void requestNowPlayingAccess() {
        try {
            Intent intent = new Intent(Settings.ACTION_NOTIFICATION_LISTENER_SETTINGS);
            intent.addFlags(Intent.FLAG_ACTIVITY_NEW_TASK);
            activity.startActivity(intent);
        } catch (Exception e) {
            android.util.Log.w("CLEANFRAME_TV", "Could not open notification access settings", e);
        }
    }

    @JavascriptInterface
    public void launchSettings() {
        activity.openTvSettings();
    }

    @JavascriptInterface
    public void launchHome() {
        try {
            Intent intent = new Intent(Intent.ACTION_MAIN);
            intent.addCategory(Intent.CATEGORY_HOME);
            intent.addFlags(Intent.FLAG_ACTIVITY_NEW_TASK);
            activity.startActivity(Intent.createChooser(intent, "Select Home"));
        } catch (Exception ignored) {}
    }

    @JavascriptInterface
    public void launchArt() {
        try {
            Intent intent = new Intent(Intent.ACTION_MAIN);
            intent.setClassName("com.android.systemui", "com.android.systemui.Somnambulator");
            intent.addFlags(Intent.FLAG_ACTIVITY_NEW_TASK);
            activity.startActivity(intent);
        } catch (Exception e) {
            try {
                Intent intent = new Intent(Settings.ACTION_DREAM_SETTINGS);
                intent.addFlags(Intent.FLAG_ACTIVITY_NEW_TASK);
                activity.startActivity(intent);
            } catch (Exception ignored) {}
        }
    }

    @JavascriptInterface
    public String getInstalledApps() {
        JSONArray array = new JSONArray();
        try {
            PackageManager pm = activity.getPackageManager();
            Set<String> seen = new HashSet<>();
            seen.add(activity.getPackageName()); // Ignore self

            // 1. First get Leanback / TV Apps
            Intent leanbackIntent = new Intent(Intent.ACTION_MAIN);
            leanbackIntent.addCategory(Intent.CATEGORY_LEANBACK_LAUNCHER);
            List<ResolveInfo> tvApps = pm.queryIntentActivities(leanbackIntent, 0);

            int rank = 10;
            if (tvApps != null) {
                for (ResolveInfo info : tvApps) {
                    if (info.activityInfo == null || info.activityInfo.packageName == null) continue;
                    String pkg = info.activityInfo.packageName;
                    if (seen.contains(pkg)) continue;
                    seen.add(pkg);

                    String name = info.loadLabel(pm).toString();
                    String brand = PACKAGE_BRANDS.containsKey(pkg) ? PACKAGE_BRANDS.get(pkg) : "";

                    JSONObject item = new JSONObject();
                    item.put("appId", pkg);
                    item.put("name", name);
                    item.put("brand", brand);
                    item.put("icon", "");
                    item.put("rank", brand.isEmpty() ? rank++ : 1);
                    array.put(item);
                }
            }

            // 2. Also check standard launcher activities
            Intent standardIntent = new Intent(Intent.ACTION_MAIN);
            standardIntent.addCategory(Intent.CATEGORY_LAUNCHER);
            List<ResolveInfo> standardApps = pm.queryIntentActivities(standardIntent, 0);

            if (standardApps != null) {
                for (ResolveInfo info : standardApps) {
                    if (info.activityInfo == null || info.activityInfo.packageName == null) continue;
                    String pkg = info.activityInfo.packageName;
                    if (seen.contains(pkg)) continue;
                    seen.add(pkg);

                    String name = info.loadLabel(pm).toString();
                    String brand = PACKAGE_BRANDS.containsKey(pkg) ? PACKAGE_BRANDS.get(pkg) : "";

                    JSONObject item = new JSONObject();
                    item.put("appId", pkg);
                    item.put("name", name);
                    item.put("brand", brand);
                    item.put("icon", "");
                    item.put("rank", brand.isEmpty() ? rank++ : 2);
                    array.put(item);
                }
            }
        } catch (Exception e) {
            e.printStackTrace();
        }
        return array.toString();
    }
}
