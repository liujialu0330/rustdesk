// ffi.kt

package ffi

import android.content.Context
import java.nio.ByteBuffer

import com.liujialu.deskviewer.RdClipboardManager

object FFI {
    init {
        System.loadLibrary("deskviewer")
    }

    external fun init(ctx: Context)
    external fun onAppStart(ctx: Context)
    external fun setClipboardManager(clipboardManager: RdClipboardManager)
    external fun translateLocale(localeName: String, input: String): String
    external fun setCodecInfo(info: String)
    external fun onClipboardUpdate(clips: ByteBuffer)
}
