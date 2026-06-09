package com.liujialu.deskviewer

/**
 * Handle events from flutter
 * Viewer-only Flutter host for the Android client.
 *
 * Inspired by [droidVNC-NG] https://github.com/bk138/droidVNC-NG
 */

import ffi.FFI

import android.content.Context
import android.content.Intent
import android.content.ClipboardManager
import android.os.Bundle
import android.os.Build
import android.provider.Settings
import android.util.Log
import android.view.WindowManager
import org.json.JSONArray
import org.json.JSONObject
import io.flutter.embedding.android.FlutterActivity
import io.flutter.embedding.engine.FlutterEngine
import io.flutter.plugin.common.MethodChannel
import kotlin.concurrent.thread


class MainActivity : FlutterActivity() {
    companion object {
        var flutterMethodChannel: MethodChannel? = null
        private var _rdClipboardManager: RdClipboardManager? = null
        val rdClipboardManager: RdClipboardManager?
            get() = _rdClipboardManager;
    }

    private val channelTag = "mChannel"
    private val logTag = "mMainActivity"
    private val allowedSettingsActions = setOf(Settings.ACTION_APPLICATION_DETAILS_SETTINGS)

    override fun configureFlutterEngine(flutterEngine: FlutterEngine) {
        super.configureFlutterEngine(flutterEngine)
        flutterMethodChannel = MethodChannel(
            flutterEngine.dartExecutor.binaryMessenger,
            channelTag
        )
        initFlutterChannel(flutterMethodChannel!!)
        thread {
            try {
                setCodecInfo()
            } catch (e: Exception) {
                Log.e("MainActivity", "Failed to setCodecInfo: ${e.message}", e)
            }
        }
    }

    override fun onResume() {
        super.onResume()
        activity.runOnUiThread {
            flutterMethodChannel?.invokeMethod(
                "on_state_changed",
                mapOf("name" to "input", "value" to false.toString())
            )
        }
    }

    override fun onActivityResult(requestCode: Int, resultCode: Int, data: Intent?) {
        super.onActivityResult(requestCode, resultCode, data)
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        if (_rdClipboardManager == null) {
            _rdClipboardManager = RdClipboardManager(getSystemService(Context.CLIPBOARD_SERVICE) as ClipboardManager)
            FFI.setClipboardManager(_rdClipboardManager!!)
        }
    }

    override fun onDestroy() {
        Log.e(logTag, "onDestroy")
        super.onDestroy()
    }

    private fun initFlutterChannel(flutterMethodChannel: MethodChannel) {
        flutterMethodChannel.setMethodCallHandler { call, result ->
            // make sure result will be invoked, otherwise flutter will await forever
            when (call.method) {
                "init_service" -> {
                    result.success(false)
                }
                "start_capture" -> {
                    result.success(false)
                }
                "stop_service" -> {
                    result.success(true)
                }
                "check_permission" -> {
                    result.success(false)
                }
                "request_permission" -> {
                    if (call.arguments is String) {
                        val permission = call.arguments as String
                        notifyPermissionDenied(permission)
                    } else {
                        notifyPermissionDenied("")
                    }
                    result.success(false)
                }
                START_ACTION -> {
                    if (call.arguments is String) {
                        val action = call.arguments as String
                        if (allowedSettingsActions.contains(action)) {
                            startAction(context, action)
                            result.success(true)
                        } else {
                            result.success(false)
                        }
                    } else {
                        result.success(false)
                    }
                }
                "check_video_permission" -> {
                    result.success(false)
                }
                "check_service" -> {
                    Companion.flutterMethodChannel?.invokeMethod(
                        "on_state_changed",
                        mapOf("name" to "input", "value" to false.toString())
                    )
                    Companion.flutterMethodChannel?.invokeMethod(
                        "on_state_changed",
                        mapOf("name" to "media", "value" to false.toString())
                    )
                    result.success(true)
                }
                "stop_input" -> {
                    Companion.flutterMethodChannel?.invokeMethod(
                        "on_state_changed",
                        mapOf("name" to "input", "value" to false.toString())
                    )
                    result.success(true)
                }
                "cancel_notification" -> {
                    result.success(true)
                }
                "enable_soft_keyboard" -> {
                    // https://blog.csdn.net/hanye2020/article/details/105553780
                    if (call.arguments as Boolean) {
                        window.clearFlags(WindowManager.LayoutParams.FLAG_ALT_FOCUSABLE_IM)
                    } else {
                        window.addFlags(WindowManager.LayoutParams.FLAG_ALT_FOCUSABLE_IM)
                    }
                    result.success(true)

                }
                "try_sync_clipboard" -> {
                    rdClipboardManager?.syncClipboard(true)
                    result.success(true)
                }
                GET_START_ON_BOOT_OPT -> {
                    result.success(false)
                }
                SET_START_ON_BOOT_OPT -> {
                    result.success(call.arguments == false)
                }
                SYNC_APP_DIR_CONFIG_PATH -> {
                    if (call.arguments is String) {
                        val prefs = getSharedPreferences(KEY_SHARED_PREFERENCES, MODE_PRIVATE)
                        val edit = prefs.edit()
                        edit.putString(KEY_APP_DIR_CONFIG_PATH, call.arguments as String)
                        edit.apply()
                        result.success(true)
                    } else {
                        result.success(false)
                    }
                }
                GET_VALUE -> {
                    if (call.arguments is String) {
                        if (call.arguments == KEY_IS_SUPPORT_VOICE_CALL) {
                            result.success(false)
                        } else {
                            result.error("-1", "No such key", null)
                        }
                    } else {
                        result.success(null)
                    }
                }
                "on_voice_call_started" -> {
                    result.success(false)
                }
                "on_voice_call_closed" -> {
                    result.success(false)
                }
                else -> {
                    result.error("-1", "No such method", null)
                }
            }
        }
    }

    private fun notifyPermissionDenied(permission: String) {
        flutterMethodChannel?.invokeMethod(
            "on_android_permission_result",
            mapOf("type" to permission, "result" to false)
        )
    }

    private fun setCodecInfo() {
        val windowManager = getSystemService(Context.WINDOW_SERVICE) as WindowManager
        val wh = getScreenSize(windowManager)
        val result = JSONObject()
        result.put("version", Build.VERSION.SDK_INT)
        result.put("w", wh.first)
        result.put("h", wh.second)
        result.put("codecs", JSONArray())
        FFI.setCodecInfo(result.toString())
    }

    override fun onStop() {
        super.onStop()
    }

    override fun onStart() {
        super.onStart()
    }
}
