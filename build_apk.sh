#!/usr/bin/env bash
set -e

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SDK_ROOT="/Users/dexter/Library/Android/sdk"
BUILD_TOOLS="$SDK_ROOT/build-tools/36.0.0"
PLATFORM="$SDK_ROOT/platforms/android-37.0"
ANDROID_JAR="$PLATFORM/android.jar"

# Detect Java
if [ -d "/opt/homebrew/opt/openjdk@21/libexec/openjdk.jdk/Contents/Home" ]; then
    export JAVA_HOME="/opt/homebrew/opt/openjdk@21/libexec/openjdk.jdk/Contents/Home"
elif [ -d "/opt/homebrew/opt/openjdk/libexec/openjdk.jdk/Contents/Home" ]; then
    export JAVA_HOME="/opt/homebrew/opt/openjdk/libexec/openjdk.jdk/Contents/Home"
fi
export PATH="$JAVA_HOME/bin:$BUILD_TOOLS:$PATH"

echo "=== Using Java: $(which java || true) ==="
echo "=== Android Build Tools: $BUILD_TOOLS ==="

WORK_DIR="$DIR/build"
rm -rf "$WORK_DIR"
mkdir -p "$WORK_DIR/compiled_res" "$WORK_DIR/gen" "$WORK_DIR/classes" "$WORK_DIR/dex"

# 1. Compile resources with aapt2
echo "--> Compiling Android resources..."
"$BUILD_TOOLS/aapt2" compile --dir "$DIR/app/src/main/res" -o "$WORK_DIR/compiled_res.zip"

# 2. Link resources with aapt2 and generate R.java
echo "--> Linking resources and generating R.java..."
"$BUILD_TOOLS/aapt2" link \
    -I "$ANDROID_JAR" \
    --manifest "$DIR/app/src/main/AndroidManifest.xml" \
    --min-sdk-version 21 \
    --target-sdk-version 34 \
    --java "$WORK_DIR/gen" \
    -o "$WORK_DIR/app-base.apk" \
    "$WORK_DIR/compiled_res.zip"

# 3. Compile Java sources
echo "--> Compiling Java sources with javac..."
"$JAVA_HOME/bin/javac" \
    -cp "$ANDROID_JAR" \
    -d "$WORK_DIR/classes" \
    -source 17 -target 17 \
    $(find "$WORK_DIR/gen" "$DIR/app/src/main/java" -name "*.java")

# 4. Convert classes to DEX with d8
echo "--> Converting bytecode to DEX with d8..."
"$BUILD_TOOLS/d8" \
    --min-api 21 \
    --lib "$ANDROID_JAR" \
    --output "$WORK_DIR/dex" \
    $(find "$WORK_DIR/classes" -name "*.class")

# 5. Package DEX into APK
echo "--> Adding classes.dex, assets, and native libs to APK..."
cp "$WORK_DIR/app-base.apk" "$WORK_DIR/app-unaligned.apk"
cd "$WORK_DIR/dex"
zip -0 -j "$WORK_DIR/app-unaligned.apk" classes.dex

# Add assets directory to APK
cd "$DIR/app/src/main"
zip -ur "$WORK_DIR/app-unaligned.apk" assets/

# Add native libraries (jniLibs) to APK
if [ -d "$DIR/app/src/main/jniLibs" ] && [ "$(ls -A "$DIR/app/src/main/jniLibs")" ]; then
    echo "--> Packaging native libraries (jniLibs) into lib/..."
    mkdir -p "$WORK_DIR/apk_root/lib"
    cp -r "$DIR/app/src/main/jniLibs/"* "$WORK_DIR/apk_root/lib/"
    cd "$WORK_DIR/apk_root"
    zip -0 -ur "$WORK_DIR/app-unaligned.apk" lib/
fi

# 6. Zipalign APK
echo "--> Aligning APK with zipalign..."
cd "$DIR"
"$BUILD_TOOLS/zipalign" -f -v -p 4 "$WORK_DIR/app-unaligned.apk" "$DIR/cleanframe-tv-aligned.apk"

# 7. Generate debug key if not exists and sign
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

echo "--> Signing APK with apksigner..."
"$BUILD_TOOLS/apksigner" sign \
    --ks "$KEYSTORE" \
    --ks-pass pass:android \
    --key-pass pass:android \
    --ks-key-alias androiddebugkey \
    --out "$DIR/cleanframe-tv.apk" \
    "$DIR/cleanframe-tv-aligned.apk"

echo "=== SUCCESS! CleanFrame TV APK generated: $DIR/cleanframe-tv.apk ==="
ls -lh "$DIR/cleanframe-tv.apk"
