# Android Viewer-Only Fork Plan

## Background

This fork is for personal use on an Android phone to control trusted Windows
computers through a self-hosted RustDesk server. The Android app should act as a
viewer/controller only. It should not expose the phone as a remotely controlled
device.

## Safety Boundary

This plan does not implement stealth, obfuscation, anti-analysis, signature
spoofing, or malware-detection bypasses. The goal is a transparent personal fork
with a distinct app identity and a smaller permission and feature surface.

## Goals

- Keep Android phone-to-PC remote-control capability.
- Remove Android phone-as-host/server functionality from the user-facing app.
- Remove Android permissions and manifest components only needed for controlling
  the phone.
- Keep networking and connection UI needed to connect to a remote Windows host.
- Use a distinct package/application identity for the personal fork.
- Produce verifiable APK permission and component reports after build.

## Non-Goals

- Do not bypass Play Protect or other mobile security products.
- Do not hide behavior through packers, string encryption, or anti-debug logic.
- Do not remove license obligations or upstream attribution from repository docs.
- Do not change desktop/server RustDesk behavior.
- Do not publish this APK as an official RustDesk build.

## Planned App Identity Changes

- Change Android `applicationId` from `com.carriez.flutter_hbb` to a personal
  fork package, currently planned as `com.liujialu.deskviewer`.
- Change Android app label from `RustDesk` to `Desk Viewer`.
- Change Android deep-link scheme from `rustdesk` to `deskviewer`.
- Keep source package paths unchanged initially to avoid a broad Kotlin package
  migration. Gradle `applicationId` is enough for APK identity.

## Planned Permission Removals

Remove permissions tied to Android phone-host operation:

- `android.permission.MANAGE_EXTERNAL_STORAGE`
- `android.permission.READ_EXTERNAL_STORAGE`
- `android.permission.WRITE_EXTERNAL_STORAGE`
- `android.permission.RECORD_AUDIO`
- `android.permission.FOREGROUND_SERVICE`
- `android.permission.WAKE_LOCK`
- `android.permission.RECEIVE_BOOT_COMPLETED`
- `android.permission.SYSTEM_ALERT_WINDOW`
- `android.permission.REQUEST_IGNORE_BATTERY_OPTIMIZATIONS`

Keep only the baseline permissions needed for a viewer client:

- `android.permission.INTERNET`
- `android.permission.ACCESS_NETWORK_STATE`

`android.permission.POST_NOTIFICATIONS` is not required for viewer-only use and
is planned for removal unless build/runtime verification shows a hard dependency.

## Planned Manifest Component Removals

Remove Android components needed for phone-host behavior:

- `.BootReceiver`
- `.InputService`
- `.PermissionRequestTransparentActivity`
- `.MainService`
- `.FloatingWindowService`

Keep:

- `.MainApplication`
- `.MainActivity`
- Flutter embedding metadata
- Browser custom-tab query if needed by existing settings/help links

## Planned Kotlin Changes

- Convert `MainActivity` into a viewer-only Flutter host.
- Remove or no-op MethodChannel handlers that request phone-host capabilities:
  `init_service`, `start_capture`, `stop_service`, `check_video_permission`,
  `stop_input`, start-on-boot option methods, voice-call audio methods, and
  notification cancellation through `MainService`.
- Keep MethodChannel support needed by viewer UI:
  permission check/request plumbing, settings action launcher, soft keyboard
  window flags, clipboard sync, app config path storage, and feature probes.
- Restrict generic Android permission and settings MethodChannel calls through an
  allowlist so Flutter cannot request overlay, audio recording, all-files
  storage, accessibility settings, notifications, boot, battery optimization, or
  screen-capture setup from another path.
- Avoid deleting Kotlin service files in the first pass; removing them from the
  manifest and removing references from `MainActivity` is lower-risk and keeps
  the patch reviewable.

## Planned Flutter UI/Model Changes

- Add a single viewer-only guard, `kAndroidViewerOnly`, and use it consistently
  with `isAndroid`.
- Remove the Android `ServerPage` and chat/server tabs from the mobile home page.
- Keep the `ConnectionPage` as the primary mobile page.
- In viewer-only mode, force Android mobile navigation to expose the connection
  page even if an incoming-only option is present in upstream config.
- Prevent viewer-only Android builds from requesting host-side permissions:
  screen sharing, accessibility input service, floating window, microphone,
  full storage, boot startup, and notification permissions for host sessions.
- Hide or disable settings entries related to phone-host features:
  share screen, start service, input permission, file transfer as host,
  audio capture as host, floating window, start on boot, keep screen on while
  controlled.
- Add `ServerModel` guards so hidden host features cannot still run through
  model/event paths:
  - service start/stop must no-op;
  - incoming host sessions must be rejected/closed;
  - `start_capture` must not be invoked;
  - host audio/file/input permission toggles must no-op;
  - Android keep-screen-on handling for controlled-side sessions must no-op.
- Disable mobile file-transfer entries/deep links for the first pass because the
  current implementation can request full external-storage access. Revisit later
  only with app-scoped storage or Android SAF.
- Keep remote-control features needed to operate the Windows computer, including
  keyboard/mouse input to the remote host, clipboard sync if it works without
  sensitive permissions, and connection history.

## Verification Plan

1. Static checks:
   - `rg` for removed permissions and components in Android manifest/build code.
   - `rg` for host launch paths:
     `init_service`, `start_capture`, `mainStartService`, `startServer`,
     `MainService`, `InputService`, `FloatingWindowService`.
   - `flutter analyze` if local Flutter SDK is available.
   - Gradle manifest merge check if Android tooling is available.
2. Build:
   - Prefer `flutter build apk --debug --target-platform android-arm64` for a
     quick local build when native libraries/toolchains are present.
   - If native Android dependencies are missing locally, document the exact
     blocker and use repository CI build instructions as the next path.
3. APK inspection:
   - Use `aapt dump permissions` or `apkanalyzer manifest permissions`.
   - Use `aapt dump xmltree` or `apkanalyzer manifest print` to confirm removed
     services/receiver/activity do not appear.
   - Inspect the merged manifest because Flutter plugins may reintroduce
     permissions such as camera, storage/media, notifications, or wakelock.
     Remove merged plugin permissions with manifest rules or disable the
     corresponding UI paths if they are not viewer-essential.
4. Runtime smoke test:
   - Install on Android test device if available.
   - Confirm app opens to connection UI.
   - Confirm it does not ask for accessibility, screen capture, overlay,
     microphone, full storage, boot, or battery-optimization permissions.
   - Connect to a trusted Windows host through the self-hosted server.

## Risks

- RustDesk Android viewer and host code are intertwined; some host-side classes
  may still compile but remain unreachable.
- Removing `ServerPage` can affect chat overlays for incoming sessions; this is
  acceptable for viewer-only use.
- Removing `WAKE_LOCK` can affect long outgoing control sessions if Flutter's
  wakelock helper relies on the manifest permission. If runtime testing shows
  screen-awake behavior is important and no sensitive prompt is introduced,
  re-adding this normal permission is acceptable.
- The Rust native library may still contain host-side code even if the Android UI
  no longer exposes it. A deeper Rust feature-level split can be planned later.
- Security products may still warn about remote-control capabilities even after
  permissions are minimized. This plan reduces sensitive behavior; it does not
  guarantee any scanner decision.
