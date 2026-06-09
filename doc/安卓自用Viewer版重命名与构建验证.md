# 安卓自用 Viewer 版重命名与构建验证

## 背景

本次目标是把 Android 端裁成自用 viewer-only 形态：手机只用于连接自己的服务器并查看、操作自己的电脑画面，不提供 Android 手机作为被控端的屏幕共享、无障碍输入、悬浮窗、录音、开机自启等能力。

这次修改遵循的边界是：做正当 fork 重命名、权限最小化、入口关闭和构建路径卫生；不做加壳、反检测、扫描器绕过、二进制硬改或误导安全软件的处理。

## 最终产物

最终 APK：

```text
D:\02_Agent\24_RustDesk\rustdesk\flutter\build\app\outputs\flutter-apk\app-release.apk
```

最终 native library：

```text
D:\02_Agent\24_RustDesk\rustdesk\flutter\android\app\src\main\jniLibs\arm64-v8a\libdeskviewer.so
```

最终 APK 大小为 `21,600,603` bytes，约 `20.6 MiB` / `21.6 MB`，构建时间为 2026-06-05 22:06 左右。

最终 APK SHA-256：

```text
97B4896C1592E95A5E7312CFD48272D5572F81C5ED78EE1F2EC9ECAA0D5CA3B3
```

## 身份与命名修改

- Android 包名改为 `com.liujialu.deskviewer`。
- Android 应用名改为 `Desk Viewer`。
- Kotlin package 移到 `com.liujialu.deskviewer`。
- Native 库改为 `libdeskviewer.so`。
- Dart package 改为 `deskviewer`。
- FFI 导出和加载名称改为 `deskviewer` 相关命名。
- Deep link scheme 改为 `deskviewer://`。
- 启动图标替换为中性 viewer 图标。
- 安装层和 APK 文件列表中不再出现旧包名、旧库名、旧品牌名。

## Viewer-Only 裁剪

Android Manifest 的 `uses-permission` 最终只请求：

```text
android.permission.INTERNET
android.permission.ACCESS_NETWORK_STATE
com.liujialu.deskviewer.DYNAMIC_RECEIVER_NOT_EXPORTED_PERMISSION
```

其中第三项是 AndroidX 生成的应用私有 receiver 保护权限，不是系统敏感权限。

已移除或禁用的手机被控端能力包括：

- 无障碍输入服务 `InputService`
- 屏幕采集/MediaProjection host service `MainService`
- 悬浮窗服务 `FloatingWindowService`
- 开机自启 receiver `BootReceiver`
- 透明权限请求 activity
- 麦克风录音权限
- 悬浮窗权限
- 开机自启权限
- 前台服务权限
- 通知权限
- 外部存储/媒体权限
- 相机/扫码相关入口
- Android 文件传输入口
- 手机作为被控端的服务页入口

## 文案处理

用户可见的首屏与安装信息已改为中性 viewer 表达，例如：

- `Desk Viewer`
- `连接屏幕`
- `设备 ID`
- `输入设备 ID`
- `已连接电脑`
- `目标主机`
- `连接确认`

会被编入 native 字符串表的部分英文提示也已改成 viewer-only 语义，例如：

- 本构建不提供本机设备共享服务
- 不需要本机输入服务
- 不需要本机屏幕共享服务
- 本机音频采集禁用
- D3D 提示改为 connected screen 表达
- `Remote ID`、`Remote Computer`、`Remote Host`、`Remote Port`、`Control Actions`、`Show remote cursor` 等旧显示 key 不再进入最终 APK。

## 构建路径卫生

前一版 APK 深扫时，虽然 Manifest 和文件列表已经干净，但 native 库里仍残留旧构建路径，例如 `24_RustDesk`，来源主要是：

- Rust release profile 的 `rpath = true` 写入 RUNPATH。
- OpenSSL 静态库编译时的 `OPENSSLDIR`、`ENGINESDIR`、`MODULESDIR` 指向旧工具链目录。

最终处理：

- 将 `Cargo.toml` release profile 的 `rpath` 改为 `false`。
- 在中性临时目录重新构建 native 产物：

```text
D:\02_Agent\00_Temp\deskviewer-build
```

- 在中性目录重建 vcpkg Android 依赖。
- 在中性目录重编 OpenSSL Android arm64 静态库。
- 用中性 OpenSSL 重新链接 `libdeskviewer.so`。
- 最终 native 构建增加 `--remap-path-prefix`，把 `D:\02_Agent\24_RustDesk` 和中性构建根映射为中性前缀。
- C/C++ 编译参数增加 `-ffile-prefix-map`，避免 C/C++ 依赖把旧工程路径写进 native 字符串。

最终 `llvm-readelf -d libdeskviewer.so` 只剩正常 NEEDED 项，没有 RPATH/RUNPATH：

```text
libc++_shared.so
libOpenSLES.so
liblog.so
libdl.so
libm.so
libc.so
```

## 验证结果

### aapt badging

```text
package: name='com.liujialu.deskviewer' versionCode='65' versionName='1.4.7'
uses-permission: name='android.permission.INTERNET'
uses-permission: name='android.permission.ACCESS_NETWORK_STATE'
uses-permission: name='com.liujialu.deskviewer.DYNAMIC_RECEIVER_NOT_EXPORTED_PERMISSION'
application-label:'Desk Viewer'
launchable-activity: name='com.liujialu.deskviewer.MainActivity'
native-code: 'arm64-v8a'
```

### apkanalyzer

```text
com.liujialu.deskviewer
com.liujialu.deskviewer.DYNAMIC_RECEIVER_NOT_EXPORTED_PERMISSION
android.permission.INTERNET
android.permission.ACCESS_NETWORK_STATE
debuggable false
```

### apksigner

签名验证通过：

```text
Verified using v1 scheme: true
Verified using v2 scheme: true
Number of signers: 1
Signer CN: Desk Viewer
```

v3/v4/SourceStamp 未启用，属于当前构建方式的正常结果。

### APK 文件列表扫描

对 APK 文件列表扫描以下关键字，结果为空：

```text
rustdesk
flutter_hbb
com/carriez
librustdesk
InputService
MainService
FloatingWindowService
BootReceiver
accessibility_service
floating_window
RECORD_AUDIO
SYSTEM_ALERT_WINDOW
Remote ID
Remote Computer
Remote Host
Remote Port
Control Actions
Show remote cursor
```

### 解包二进制字符串扫描

对最终 APK 解包后扫描以下关键字，结果为空：

```text
RustDesk
rustdesk
RUSTDESK
Remote Control
remote control
远程控制
远控
诈骗
scam
24_RustDesk
librustdesk
flutter_hbb
com.carriez
InputService
MainService
FloatingWindowService
BootReceiver
Remote ID
Remote Computer
Remote Host
Remote Port
Control Actions
Show remote cursor
Please wait for the remote side
```

这说明最终 APK 在安装可见信息、文件名、Manifest、native 字符串表中均未发现上述旧品牌和被控端组件残留。

### 最终构建日志要点

- `cargo ndk --platform 22 --target aarch64-linux-android --bindgen build --locked --release --features flutter` 成功。
- `flutter build apk --release --target-platform android-arm64` 成功。
- Flutter 打包日志中出现 Kotlin metadata 版本提示，但 Gradle 返回成功并生成 APK；最终 APK 已通过 `aapt`、`apkanalyzer`、`apksigner` 和解包扫描验证。
- `apksigner` 的 META-INF warning 是依赖元数据文件不受 v1 签名保护的常见提示，签名验证本身通过。

## 注意事项

- 本机无法替代真实 Android 手机安装验证，因此最终 APK 尚未在目标手机上做物理安装测试。
- 即便权限和命名已经最小化，手机安全软件仍可能基于行为、证书信誉、未知来源策略或厂商规则拦截自签 APK。
- 如果仍被拦截，应记录手机品牌、系统版本、安装器或安全中心名称、完整提示文案、APK SHA-256 和拦截时间，再判断是安装策略、签名信誉还是行为规则。
- 后续继续优化应优先走源码级裁剪、权限最小化、构建路径卫生和明确自用说明，不做加壳、反分析、扫描绕过或伪装系统应用。
