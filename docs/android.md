# Android Build Guide

## One-time Setup

### 1. Android Studio
Install the latest stable version from developer.android.com.

### 2. SDK, NDK, CMake
In Android Studio → Settings → SDK Manager → SDK Tools, install:
- Android SDK Platform 33 (or higher)
- NDK (Side by side) — version **25.x**
- CMake

### 3. Environment Variables (Windows)
Set these as permanent user environment variables:

```
JAVA_HOME    = C:\Program Files\Android\Android Studio\jbr
ANDROID_HOME = %LOCALAPPDATA%\Android\Sdk
NDK_HOME     = %LOCALAPPDATA%\Android\Sdk\ndk\25.x.xxxxxxx   ← exact version from SDK Manager
```

Add to PATH: `%ANDROID_HOME%\platform-tools`

Restart your terminal after setting these.

### 4. Rust Android Targets

```bash
rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
```

### 5. Physical Device
Enable USB debugging on your phone (Settings → Developer Options), plug in via USB, then verify:

```bash
adb devices
# Must show your device as "device", not "unauthorized"
```

---

## Development (live on device)

```bash
npm run android
# = dx serve --platform android --package mobile
```

`dx` compiles the Rust library, generates the Android project under `target/dx/mobile/debug/android/`, starts the fullstack backend locally, and installs + launches the app on the connected device via `adb`. The server URL is configured automatically so server functions reach your dev machine. Hot reload works.

---

## Standalone Debug APK

```bash
npm run android:build
# = dx bundle --platform android --package mobile
```

APK output:
```
target/dx/mobile/debug/android/app/app/build/outputs/apk/debug/app-debug.apk
```

Install manually:
```bash
adb install target/dx/mobile/debug/android/app/app/build/outputs/apk/debug/app-debug.apk
```

> ⚠️ With a standalone APK the app calls server functions against a hardcoded URL. For testing against a local backend, use `npm run android` instead.

---

## Known Issue — Java Compatibility (release builds only)

`dx` generates gradle files with `jvmTarget = "1.8"`, which fails with Java 21 on modern Android Studio. **Debug builds are unaffected.** If you hit this on a release build, patch the generated file after `dx bundle`:

File: `target/dx/mobile/debug/android/app/app/build.gradle.kts`

```diff
- jvmTarget = "1.8"
+ jvmTarget = "17"
```

This file is regenerated on each `dx bundle` run, so the patch must be reapplied each time.

---

## App Configuration

`packages/mobile/Dioxus.toml` controls the app metadata:

```toml
[application]
name = "Counted"
default_platform = "android"

[bundle]
identifier = "fr.counted.app"
publisher = "Counted"
icon = ["assets/favicon.ico"]

[bundle.android]
min_sdk_version = 24
target_sdk_version = 33
```
