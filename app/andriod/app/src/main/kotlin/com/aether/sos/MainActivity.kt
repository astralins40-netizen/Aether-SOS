package com.aether.sos

import android.os.Bundle
import io.flutter.embedding.android.FlutterActivity
import io.flutter.embedding.engine.FlutterEngine
import io.flutter.plugin.common.MethodChannel

class MainActivity: FlutterActivity() {
    private val CHANNEL = "com.aether.pulse/nan"
    private var nanService: WifiNanService? = null

    override fun configureFlutterEngine(flutterEngine: FlutterEngine) {
        super.configureFlutterEngine(flutterEngine)
        
        nanService = WifiNanService(context)

        MethodChannel(flutterEngine.dartExecutor.binaryMessenger, CHANNEL).setMethodCallHandler { call, result ->
            when (call.method) {
                "startPulse" -> {
                    val payload = call.argument<ByteArray>("payload")
                    if (payload != null) {
                        nanService?.startPulse(payload)
                        result.success(null)
                    } else {
                        result.error("INVALID_PAYLOAD", "Payload cannot be null", null)
                    }
                }
                "stopPulse" -> {
                    nanService?.stopPulse()
                    result.success(null)
                }
                else -> {
                    result.notImplemented()
                }
            }
        }
    }

    override fun onDestroy() {
        nanService?.stopPulse()
        super.onDestroy()
    }
}
