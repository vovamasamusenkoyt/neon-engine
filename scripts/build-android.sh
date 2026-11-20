#!/usr/bin/env bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$ROOT_DIR"

export ANDROID_NDK_HOME=/opt/android-ndk
BUILD_TOOLS="/home/vmko/.android-sdk/build-tools/35.0.0"
PLATFORM_JAR="/home/vmko/.android-sdk/platforms/android-35/android.jar"
KEYSTORE="/home/vmko/.android/debug.keystore"

echo "=== Building neon-app for Android (aarch64-linux-android) ==="
cargo ndk -t arm64-v8a -o ./jniLibs build -p neon-app --release

echo "=== Packaging APK with aapt2 ==="
rm -rf base.apk neon_engine.apk
"$BUILD_TOOLS/aapt2" link -o base.apk -I "$PLATFORM_JAR" --manifest AndroidManifest.xml

python3 -c '
import zipfile
base_apk = "base.apk"
so_path = "jniLibs/arm64-v8a/libneon_app.so"
with zipfile.ZipFile(base_apk, "a", compression=zipfile.ZIP_DEFLATED) as z:
    z.write(so_path, "lib/arm64-v8a/libneon_app.so")
'

echo "=== Aligning and Signing APK ==="
"$BUILD_TOOLS/zipalign" -v -p 4 base.apk neon_engine.apk
"$BUILD_TOOLS/apksigner" sign --ks "$KEYSTORE" --ks-pass pass:android --key-pass pass:android neon_engine.apk

echo "=== Deploying to connected Android device ==="
adb install -r neon_engine.apk
adb shell "input keyevent KEYCODE_WAKEUP && wm dismiss-keyguard"
adb logcat -c
adb shell "am force-stop com.neon.engine"
adb shell "am start -n com.neon.engine/android.app.NativeActivity"

sleep 2
echo "=== Device Logcat (NeonEngine) ==="
adb logcat -d -s NeonEngine | tail -n 15
