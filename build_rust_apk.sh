#!/usr/bin/env bash
set -e

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SDK_ROOT="/Users/dexter/Library/Android/sdk"
BUILD_TOOLS="$SDK_ROOT/build-tools/36.0.0"
PLATFORM="$SDK_ROOT/platforms/android-37.0"
export ANDROID_JAR="$PLATFORM/android.jar"

if [ -d "/opt/homebrew/opt/openjdk@21/libexec/openjdk.jdk/Contents/Home" ]; then
    export JAVA_HOME="/opt/homebrew/opt/openjdk@21/libexec/openjdk.jdk/Contents/Home"
elif [ -d "/opt/homebrew/opt/openjdk/libexec/openjdk.jdk/Contents/Home" ]; then
    export JAVA_HOME="/opt/homebrew/opt/openjdk/libexec/openjdk.jdk/Contents/Home"
fi
export PATH="/opt/homebrew/bin:$HOME/.cargo/bin:$JAVA_HOME/bin:$BUILD_TOOLS:$PATH"
export ANDROID_NDK="/Users/dexter/Library/Android/sdk/ndk/26.3.11579264"
export ANDROID_NDK_HOME="$ANDROID_NDK"
export NDK_HOME="$ANDROID_NDK"
export ANDROID_HOME="$SDK_ROOT"
export ANDROID_SDK_ROOT="$SDK_ROOT"

WORK_DIR="$DIR/build-rust"
rm -rf "$WORK_DIR"
mkdir -p "$WORK_DIR/compiled_res" "$WORK_DIR/apk_root/lib/arm64-v8a" "$WORK_DIR/apk_root/lib/armeabi-v7a"

echo "=== Building CleanFrame Native Rust libraries ==="
cd "$DIR/cleanframe-native"
cargo ndk -t arm64-v8a -t armeabi-v7a -o "$WORK_DIR/apk_root/lib" build --release

echo "=== Packaging CleanFrame Native APK ==="
cd "$DIR"
"$BUILD_TOOLS/aapt2" compile --dir "$DIR/app/src/main/res" -o "$WORK_DIR/compiled_res.zip"

"$BUILD_TOOLS/aapt2" link \
    -I "$ANDROID_JAR" \
    --manifest "$DIR/cleanframe-native/AndroidManifest.xml" \
    --min-sdk-version 21 \
    --target-sdk-version 34 \
    -o "$WORK_DIR/app-base.apk" \
    "$WORK_DIR/compiled_res.zip"

cp "$WORK_DIR/app-base.apk" "$WORK_DIR/app-unaligned.apk"

DEX_FILE=$(find "$DIR/cleanframe-native/target" -name "classes.dex" | head -n 1)
if [ -n "$DEX_FILE" ] && [ -f "$DEX_FILE" ]; then
    echo "--> Found Slint helper classes.dex: $DEX_FILE"
    cp "$DEX_FILE" "$WORK_DIR/apk_root/classes.dex"
    cd "$WORK_DIR/apk_root"
    zip -0 -ur "$WORK_DIR/app-unaligned.apk" lib/ classes.dex
else
    echo "--> Warning: No classes.dex found, packaging lib/ only"
    cd "$WORK_DIR/apk_root"
    zip -0 -ur "$WORK_DIR/app-unaligned.apk" lib/
fi

cd "$DIR"
"$BUILD_TOOLS/zipalign" -f -v -p 4 "$WORK_DIR/app-unaligned.apk" "$WORK_DIR/cleanframe-rust-aligned.apk"

KEYSTORE="$DIR/debug.keystore"
if [ ! -f "$KEYSTORE" ]; then
    echo "--> Creating debug keystore..."
    "$JAVA_HOME/bin/keytool" -genkey -v \
        -keystore "$KEYSTORE" \
        -alias androiddebugkey \
        -keyalg RSA -keysize 2048 -validity 10000 \
        -storepass android -keypass android \
        -dname "CN=Android Debug,O=Android,C=US"
fi

"$BUILD_TOOLS/apksigner" sign \
    --ks "$KEYSTORE" \
    --ks-pass pass:android \
    --key-pass pass:android \
    --ks-key-alias androiddebugkey \
    --out "$DIR/cleanframe-rust.apk" \
    "$WORK_DIR/cleanframe-rust-aligned.apk"

echo "=== SUCCESS: $DIR/cleanframe-rust.apk generated ==="
ls -lh "$DIR/cleanframe-rust.apk"

if [ "$1" == "--emulator" ] || [ "$1" == "-e" ]; then
    echo "--> Installing on emulator-5554..."
    adb -s emulator-5554 install -r "$DIR/cleanframe-rust.apk"
    adb -s emulator-5554 shell am force-stop com.cleanframe.tv
    adb -s emulator-5554 shell am start -n com.cleanframe.tv/android.app.NativeActivity
    echo "--> Running on emulator-5554!"
fi
