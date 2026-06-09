# Android Viewer-Only Implementation Notes

## Review Inputs

Two sub-agent reviews were run before implementation:

- Android/Manifest review: requested closing the loop across manifest,
  MethodChannel, Dart host state, native host launch paths, and merged manifest
  verification.
- Flutter/mobile review: requested a unified viewer-only guard, removal of
  mobile server/chat tabs, ServerModel short-circuiting, file-transfer blocking,
  and SettingsPage host-permission guards.

Both reviews rejected stealth or scanner-bypass work and focused on transparent
permission minimization and host feature removal.

## Implemented Changes

- Android app identity:
  - `applicationId` changed to `com.liujialu.deskviewer`.
  - App label changed to `Desk Viewer`.
  - Deep-link scheme changed to `deskviewer`.
- Android manifest:
  - Kept only `INTERNET` and `ACCESS_NETWORK_STATE`.
  - Added manifest merge removal rules for common plugin permissions such as
    camera, storage/media, audio recording, notifications, foreground service,
    wakelock, boot, overlay, and battery-optimization permissions.
  - Removed `requestLegacyExternalStorage`.
  - Removed host-side receiver/service/activity declarations:
    `.BootReceiver`, `.InputService`, `.PermissionRequestTransparentActivity`,
    `.MainService`, and `.FloatingWindowService`.
- Kotlin `MainActivity`:
  - Removed direct references to host services and input service.
  - Host MethodChannel methods now no-op: `init_service`, `start_capture`,
    `stop_service`, `check_video_permission`, `stop_input`, notification cancel,
    and voice-call audio methods.
  - Added allowlists for runtime permission and settings-action channel calls.
    The viewer-only build currently allows no runtime permission requests and
    only allows opening the app details settings page.
  - Disallowed permission requests immediately report a failed permission result
    back to Flutter so callers do not wait for a timeout.
- Flutter mobile app:
  - Added `kAndroidViewerOnly`.
  - Mobile home page now exposes connection/settings only.
  - Moved Android channel handling to `mobile/android_channel.dart` so `main.dart`
    no longer imports the Android `ServerPage`.
  - Android channel now handles only viewer-safe callbacks.
  - `ServerModel` rejects incoming host sessions and short-circuits host service,
    input, file, audio, notification, floating-window, voice-call, and controlled
    keep-awake paths in Android viewer-only mode.
  - Mobile file transfer is blocked in Android viewer-only mode because the
    existing path requests all-files storage.
  - SettingsPage skips host-side permission checks and hides host sections in
    Android viewer-only mode.
  - SettingsPage removes the QR scan action. `qr_code_scanner`, `zxing2`, and
    `image_picker` were removed from `pubspec.yaml`.
  - Android viewer-only mode makes `WakelockManager` a no-op so `WAKE_LOCK`
    does not need to be declared.
- Android build surface:
  - Host-only Kotlin sources are excluded from the Android build:
    `MainService.kt`, `InputService.kt`, `FloatingWindowService.kt`,
    `BootReceiver.kt`, `PermissionRequestTransparentActivity.kt`,
    `AudioRecordHandle.kt`, `KeyboardKeyEventMapper.kt`, and
    `VolumeController.kt`.
  - `XXPermissions` was removed from Android dependencies because viewer-only
    mode does not request runtime permissions.
  - Android hardware codec enumeration was disabled for viewer-only mode; the
    app reports an empty encoder list to avoid enabling local screen-encoding
    behavior.
- Native Android build:
  - Android local VPX/AV1 encoders are stubbed out in viewer-only mode.
  - Android VPX/AV1 decoders no longer construct opaque bindgen config structs.
  - `scrap::codec::test_av1()` is not called on Android/iOS.
  - `build.rs` no longer pulls `hbb_common` as a build dependency, avoiding
    target library leakage into host build scripts.
  - Android native build uses manual OpenSSL/libsodium installs under
    `D:\02_Agent\24_RustDesk\toolchains`.

## Static Verification Run

Commands run from repository root:

```powershell
git diff --check
rg -n "uses-permission|android:permission|requestLegacyExternalStorage|foregroundServiceType|BOOT_COMPLETED|AccessibilityService|mediaProjection" flutter\android\app\src\main\AndroidManifest.xml
rg -n "android.permission" flutter\android\app\src\main\AndroidManifest.xml flutter\android\app\src\main\kotlin\com\carriez\flutter_hbb\MainActivity.kt flutter\android\app\src\main\res\values\strings.xml
rg -n "MainService|InputService|FloatingWindowService|BootReceiver|PermissionRequestTransparentActivity|AudioRecordHandle|MediaProjection|startService\(|bindService\(|ServiceConnection|LocalBinder|BIND_ACCESSIBILITY_SERVICE|foregroundServiceType|RECEIVE_BOOT_COMPLETED|SYSTEM_ALERT_WINDOW|MANAGE_EXTERNAL_STORAGE|RECORD_AUDIO|POST_NOTIFICATIONS|requestLegacyExternalStorage|mediaProjection" flutter\android\app\src\main\AndroidManifest.xml flutter\android\app\src\main\kotlin\com\carriez\flutter_hbb\MainActivity.kt flutter\android\app\src\main\res\values\strings.xml
```

Observed results before APK merge:

- `git diff --check` passed.
- Main manifest declares the kept permissions:
  - `android.permission.INTERNET`
  - `android.permission.ACCESS_NETWORK_STATE`
- Main manifest also contains `tools:node="remove"` rules for permissions that
  plugin manifests may otherwise merge in.
- `MainActivity` and the main manifest no longer reference `MainService`,
  `InputService`, `FloatingWindowService`, `BootReceiver`,
  `PermissionRequestTransparentActivity`, MediaProjection, accessibility
  binding, overlay, storage, audio, notification, boot, foreground service, or
  legacy external storage.

## Toolchain Installed Under `D:\02_Agent\24_RustDesk\toolchains`

- JDK `17.0.19+10`
- Flutter `3.24.5`
- Android SDK command-line tools, platforms `android-31`, `android-33`,
  `android-34`, build-tools `30.0.3` and `34.0.0`
- Android NDK `28.2.13676358`
- Rust `1.75.0`, `aarch64-linux-android` target, `cargo-ndk 3.1.2`
- Gradle `7.6.4`
- CMake `3.31.8`, Ninja `1.12.1`, NASM `2.16.03`, MSYS tools
- vcpkg plus Android `arm64-android` dependencies
- Manual Android `arm64` OpenSSL and libsodium installs

User-level environment variables were configured for `JAVA_HOME`,
`ANDROID_HOME`, `ANDROID_SDK_ROOT`, `ANDROID_NDK_HOME`, `ANDROID_NDK_ROOT`,
`RUSTUP_HOME`, `CARGO_HOME`, and `PATH`.

## Historical Debug Build Status

This section records an earlier debug build. It is kept only as historical
implementation evidence and is not the final APK to install.

Native library build passed:

```powershell
cargo ndk --platform 22 --target aarch64-linux-android --bindgen build --locked --release --features flutter
```

Flutter APK build passed:

```powershell
cd D:\02_Agent\24_RustDesk\rustdesk\flutter
flutter build apk --debug --target-platform android-arm64
```

Generated APK:

```text
D:\02_Agent\24_RustDesk\rustdesk\flutter\build\app\outputs\flutter-apk\app-debug.apk
```

APK metadata:

- Size: `106256029` bytes
- SHA256: `F772B4364EE4E1A96CF873C4967114E2FA34E64635196150072DCB5A2DB516B0`
- Application id: `com.liujialu.deskviewer`
- App label: `Desk Viewer`
- Version: `1.4.7` / code `65`
- Min SDK: `22`
- Target SDK: `33`
- Debuggable: `true` because this is a debug APK

## Current Release Artifact

Final release APK:

```text
D:\02_Agent\24_RustDesk\rustdesk\flutter\build\app\outputs\flutter-apk\app-release.apk
```

Current release APK metadata:

- Size: `21,600,603` bytes (`20.6 MiB` / `21.6 MB`)
- SHA256: `97B4896C1592E95A5E7312CFD48272D5572F81C5ED78EE1F2EC9ECAA0D5CA3B3`
- Application id: `com.liujialu.deskviewer`
- App label: `Desk Viewer`
- Debuggable: `false`
- Permissions: `INTERNET`, `ACCESS_NETWORK_STATE`, `com.liujialu.deskviewer.DYNAMIC_RECEIVER_NOT_EXPORTED_PERMISSION`

Release verification passed with `aapt`, `apkanalyzer`, `apksigner`, APK file-list scan, APK unpacked string scan, and native `libdeskviewer.so` string scan. The scan set included old brand names, old Android host component names, sensitive permission names, old build path fragments, and old visible wording such as `Remote ID`, `Remote Computer`, `Remote Host`, `Remote Port`, `Control Actions`, and `Show remote cursor`.

## Historical Debug APK Verification

The following verification output belongs to the earlier debug APK above. Some
values in this section, such as the launch activity package name, are obsolete.
Use the current release artifact summary for final installation decisions.

Commands run:

```powershell
$apk="D:\02_Agent\24_RustDesk\rustdesk\flutter\build\app\outputs\flutter-apk\app-debug.apk"
apkanalyzer manifest application-id $apk
apkanalyzer manifest permissions $apk
apkanalyzer manifest debuggable $apk
apkanalyzer manifest min-sdk $apk
apkanalyzer manifest target-sdk $apk
aapt dump badging $apk
aapt dump xmltree $apk AndroidManifest.xml
apkanalyzer dex packages --defined-only $apk
```

Observed permissions:

```text
com.liujialu.deskviewer.DYNAMIC_RECEIVER_NOT_EXPORTED_PERMISSION
android.permission.INTERNET
android.permission.ACCESS_NETWORK_STATE
```

The dynamic receiver permission is added by AndroidX as an app-scoped signature
permission. No Android sensitive runtime permission was present.

Observed launchable activity:

```text
com.carriez.flutter_hbb.MainActivity
```

Observed component scan:

- No `MainService`
- No `InputService`
- No `FloatingWindowService`
- No `BootReceiver`
- No `PermissionRequestTransparentActivity`
- No `CaptureActivity`
- No `ImagePickerFileProvider`
- No `foregroundServiceType`
- No accessibility service metadata

Observed dex scan for sensitive/removed symbols:

```text
NO_MATCHES
```

The dex scan pattern included:

```text
MainService|InputService|FloatingWindowService|BootReceiver|PermissionRequestTransparentActivity|AudioRecordHandle|VolumeController|KeyboardKeyEventMapper|MediaProjection|AudioRecord|RECORD_AUDIO|SYSTEM_ALERT_WINDOW|BIND_ACCESSIBILITY_SERVICE|CaptureActivity|ImagePickerFileProvider|zxing|barcodescanner|qr_code_scanner|imagepicker|hjq.permissions|android_needs_deploy
```

## Remaining Notes

- This build is a transparent self-use fork with a distinct package/app name
  and a minimized permission/component surface. It is not an anti-detection or
  scanner-bypass implementation.
- The APK is debug-signed. A release/self-use APK should be built with your own
  keystore before long-term use.
- Play Protect or another mobile security product can still warn about
  remote-control behavior or unknown self-signed APKs. Permission minimization
  reduces risk but does not guarantee an allow decision.
