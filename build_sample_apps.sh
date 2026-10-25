#!/usr/bin/env bash
set -e

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SDK_ROOT="/Users/dexter/Library/Android/sdk"
BUILD_TOOLS="$SDK_ROOT/build-tools/36.0.0"
PLATFORM="$SDK_ROOT/platforms/android-37.0"
ANDROID_JAR="$PLATFORM/android.jar"

if [ -d "/opt/homebrew/opt/openjdk@21/libexec/openjdk.jdk/Contents/Home" ]; then
    export JAVA_HOME="/opt/homebrew/opt/openjdk@21/libexec/openjdk.jdk/Contents/Home"
elif [ -d "/opt/homebrew/opt/openjdk/libexec/openjdk.jdk/Contents/Home" ]; then
    export JAVA_HOME="/opt/homebrew/opt/openjdk/libexec/openjdk.jdk/Contents/Home"
fi
export PATH="$JAVA_HOME/bin:$BUILD_TOOLS:$PATH"

KEYSTORE="$DIR/debug.keystore"
if [ ! -f "$KEYSTORE" ]; then
    "$JAVA_HOME/bin/keytool" -genkey -v \
        -keystore "$KEYSTORE" \
        -alias androiddebugkey \
        -keyalg RSA -keysize 2048 -validity 10000 \
        -storepass android -keypass android \
        -dname "CN=Android Debug,O=Android,C=US"
fi

build_and_install_app() {
    local PKG="$1"
    local LABEL="$2"
    local WORK_DIR="$DIR/build_sample/$PKG"
    rm -rf "$WORK_DIR"
    mkdir -p "$WORK_DIR/res/drawable" "$WORK_DIR/compiled_res" "$WORK_DIR/gen" "$WORK_DIR/classes" "$WORK_DIR/dex" "$WORK_DIR/src"

    cp -r "$DIR/app/src/main/res/"* "$WORK_DIR/res/"

    cat <<EOF > "$WORK_DIR/AndroidManifest.xml"
<?xml version="1.0" encoding="utf-8"?>
<manifest xmlns:android="http://schemas.android.com/apk/res/android"
    package="$PKG"
    android:versionCode="1"
    android:versionName="1.0.0">

    <uses-sdk android:minSdkVersion="21" android:targetSdkVersion="34" />
    <uses-feature android:name="android.software.leanback" android:required="false" />
    <uses-feature android:name="android.hardware.touchscreen" android:required="false" />

    <application
        android:label="$LABEL"
        android:icon="@drawable/ic_launcher"
        android:banner="@drawable/ic_banner">

        <activity
            android:name=".MainActivity"
            android:exported="true"
            android:label="$LABEL">
            <intent-filter>
                <action android:name="android.intent.action.MAIN" />
                <category android:name="android.intent.category.LEANBACK_LAUNCHER" />
                <category android:name="android.intent.category.LAUNCHER" />
            </intent-filter>
        </activity>
    </application>
</manifest>
EOF

    cat <<EOF > "$WORK_DIR/src/MainActivity.java"
package $PKG;
import android.app.Activity;
import android.os.Bundle;
import android.widget.TextView;
import android.view.Gravity;

public class MainActivity extends Activity {
    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        TextView tv = new TextView(this);
        tv.setText("$LABEL");
        tv.setTextSize(48);
        tv.setGravity(Gravity.CENTER);
        setContentView(tv);
    }
}
EOF

    "$BUILD_TOOLS/aapt2" compile --dir "$WORK_DIR/res" -o "$WORK_DIR/compiled_res.zip"
    "$BUILD_TOOLS/aapt2" link \
        -I "$ANDROID_JAR" \
        --manifest "$WORK_DIR/AndroidManifest.xml" \
        --min-sdk-version 21 \
        --target-sdk-version 34 \
        --java "$WORK_DIR/gen" \
        -o "$WORK_DIR/app-base.apk" \
        "$WORK_DIR/compiled_res.zip"

    "$JAVA_HOME/bin/javac" \
        -cp "$ANDROID_JAR" \
        -d "$WORK_DIR/classes" \
        -source 17 -target 17 \
        $(find "$WORK_DIR/gen" "$WORK_DIR/src" -name "*.java")

    "$BUILD_TOOLS/d8" \
        --min-api 21 \
        --lib "$ANDROID_JAR" \
        --output "$WORK_DIR/dex" \
        $(find "$WORK_DIR/classes" -name "*.class")

    cp "$WORK_DIR/app-base.apk" "$WORK_DIR/app-unaligned.apk"
    cd "$WORK_DIR/dex"
    zip -0 -j "$WORK_DIR/app-unaligned.apk" classes.dex

    cd "$DIR"
    "$BUILD_TOOLS/zipalign" -f -v -p 4 "$WORK_DIR/app-unaligned.apk" "$WORK_DIR/app-aligned.apk" > /dev/null

    "$BUILD_TOOLS/apksigner" sign \
        --ks "$KEYSTORE" \
        --ks-pass pass:android \
        --key-pass pass:android \
        --ks-key-alias androiddebugkey \
        --out "$WORK_DIR/sample.apk" \
        "$WORK_DIR/app-aligned.apk"

    echo "--> Installing $LABEL ($PKG) on emulator..."
    adb -s emulator-5554 install -r "$WORK_DIR/sample.apk"
}

chmod +x "$DIR/build_sample_apps.sh"

echo "=== Building and installing TV Sample Apps ==="
build_and_install_app "org.smarttube.stable" "SmartTube"
build_and_install_app "com.netflix.ninja" "Netflix"
build_and_install_app "com.amazon.amazonvideo.livingroom" "Prime Video"
build_and_install_app "com.disney.disneyplus" "Disney+"
build_and_install_app "com.spotify.tv.android" "Spotify"
build_and_install_app "com.plexapp.android" "Plex"
build_and_install_app "org.jellyfin.androidtv" "Jellyfin"

echo "=== ALL SAMPLE TV APPS INSTALLED SUCCESSFULLY ==="
