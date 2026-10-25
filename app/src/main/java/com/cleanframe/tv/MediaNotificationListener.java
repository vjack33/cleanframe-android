package com.cleanframe.tv;

import android.content.ComponentName;
import android.content.Context;
import android.content.pm.ApplicationInfo;
import android.graphics.Bitmap;
import android.media.MediaMetadata;
import android.media.session.MediaController;
import android.media.session.MediaSessionManager;
import android.media.session.PlaybackState;
import android.provider.Settings;
import android.service.notification.NotificationListenerService;
import android.util.Base64;

import org.json.JSONObject;

import java.io.ByteArrayOutputStream;
import java.util.List;

public class MediaNotificationListener extends NotificationListenerService {
    private static volatile MediaNotificationListener connectedService;

    @Override
    public void onListenerConnected() {
        super.onListenerConnected();
        connectedService = this;
    }

    @Override
    public void onListenerDisconnected() {
        if (connectedService == this) connectedService = null;
        super.onListenerDisconnected();
    }

    @Override
    public void onDestroy() {
        if (connectedService == this) connectedService = null;
        super.onDestroy();
    }

    public static boolean hasAccess(Context context) {
        String enabled = Settings.Secure.getString(context.getContentResolver(), "enabled_notification_listeners");
        if (enabled == null) return false;
        String component = new ComponentName(context, MediaNotificationListener.class).flattenToString();
        for (String entry : enabled.split(":")) if (component.equals(entry)) return true;
        return false;
    }

    private static List<MediaController> sessions(Context context) {
        MediaSessionManager manager = (MediaSessionManager) context.getSystemService(Context.MEDIA_SESSION_SERVICE);
        if (manager == null || !hasAccess(context)) return null;
        return manager.getActiveSessions(new ComponentName(context, MediaNotificationListener.class));
    }

    private static MediaController currentController(Context context) {
        List<MediaController> active = sessions(context);
        if (active == null || active.isEmpty()) return null;
        MediaController fallback = null;
        for (MediaController controller : active) {
            if (controller.getMetadata() == null) continue;
            PlaybackState state = controller.getPlaybackState();
            if (state != null && state.getState() == PlaybackState.STATE_PLAYING) return controller;
            if (fallback == null) fallback = controller;
        }
        return fallback;
    }

    public static String snapshot(Context context) {
        JSONObject result = new JSONObject();
        boolean access = hasAccess(context);
        try {
            result.put("access", access);
            if (!access) {
                result.put("available", false);
                return result.toString();
            }
            MediaController controller = currentController(context);
            MediaMetadata metadata = controller == null ? null : controller.getMetadata();
            if (metadata == null) {
                result.put("available", false);
                return result.toString();
            }

            PlaybackState state = controller.getPlaybackState();
            String title = metadata.getString(MediaMetadata.METADATA_KEY_DISPLAY_TITLE);
            if (empty(title)) title = metadata.getString(MediaMetadata.METADATA_KEY_TITLE);
            String artist = metadata.getString(MediaMetadata.METADATA_KEY_DISPLAY_SUBTITLE);
            if (empty(artist)) artist = metadata.getString(MediaMetadata.METADATA_KEY_ARTIST);
            if (empty(artist)) artist = metadata.getString(MediaMetadata.METADATA_KEY_ALBUM_ARTIST);

            result.put("available", !empty(title));
            result.put("title", title == null ? "" : title);
            result.put("artist", artist == null ? "" : artist);
            result.put("source", sourceName(context, controller.getPackageName()));
            result.put("packageName", controller.getPackageName());
            result.put("playing", state != null && state.getState() == PlaybackState.STATE_PLAYING);
            result.put("position", state == null ? 0 : Math.max(0, state.getPosition()));
            result.put("duration", Math.max(0, metadata.getLong(MediaMetadata.METADATA_KEY_DURATION)));
            result.put("artwork", artwork(metadata));
        } catch (Exception ignored) {
            try {
                result.put("access", access);
                result.put("available", false);
            } catch (Exception ignoredAgain) {}
        }
        return result.toString();
    }

    public static String control(Context context, String action) {
        try {
            MediaController controller = currentController(context);
            if (controller == null) return "No active media session.";
            MediaController.TransportControls controls = controller.getTransportControls();
            if ("previous".equals(action)) controls.skipToPrevious();
            else if ("next".equals(action)) controls.skipToNext();
            else if ("toggle".equals(action)) {
                PlaybackState state = controller.getPlaybackState();
                if (state != null && state.getState() == PlaybackState.STATE_PLAYING) controls.pause();
                else controls.play();
            } else return "Unknown media action.";
            return "ok";
        } catch (Exception e) {
            return "Media controls are unavailable.";
        }
    }

    private static String sourceName(Context context, String packageName) {
        try {
            ApplicationInfo info = context.getPackageManager().getApplicationInfo(packageName, 0);
            CharSequence label = context.getPackageManager().getApplicationLabel(info);
            return label == null ? packageName : label.toString();
        } catch (Exception ignored) {
            return packageName;
        }
    }

    private static boolean empty(String value) {
        return value == null || value.trim().isEmpty();
    }

    private static String artwork(MediaMetadata metadata) {
        Bitmap bitmap = metadata.getBitmap(MediaMetadata.METADATA_KEY_ART);
        if (bitmap == null) bitmap = metadata.getBitmap(MediaMetadata.METADATA_KEY_ALBUM_ART);
        if (bitmap == null) bitmap = metadata.getBitmap(MediaMetadata.METADATA_KEY_DISPLAY_ICON);
        if (bitmap == null) return "";
        try {
            int edge = Math.max(bitmap.getWidth(), bitmap.getHeight());
            if (edge > 256) {
                float scale = 256f / edge;
                bitmap = Bitmap.createScaledBitmap(bitmap, Math.max(1, Math.round(bitmap.getWidth() * scale)),
                        Math.max(1, Math.round(bitmap.getHeight() * scale)), true);
            }
            ByteArrayOutputStream output = new ByteArrayOutputStream();
            bitmap.compress(Bitmap.CompressFormat.JPEG, 78, output);
            return "data:image/jpeg;base64," + Base64.encodeToString(output.toByteArray(), Base64.NO_WRAP);
        } catch (Exception ignored) {
            return "";
        }
    }
}