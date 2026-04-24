package com.aether.sos

import android.content.Context
import android.net.wifi.aware.*
import android.os.Handler
import android.os.Looper
import android.util.Log

/**
 * Aether Android Pulse - 實作 Wi-Fi NAN (Neighbor Awareness Networking) 的無握手廣播
 * 完全依賴 Wi-Fi Aware 的 Service Discovery Indicator (SDI) 夾帶 256 Bytes 封包
 */
class WifiNanService(private val context: Context) {
    private val TAG = "AetherWifiNan"
    private val SERVICE_NAME = "AetherMesh_v1"
    
    private var wifiAwareSession: WifiAwareSession? = null
    private var publishSession: PublishDiscoverySession? = null
    private var subscribeSession: SubscribeDiscoverySession? = null

    // 取得系統的 WifiAwareManager
    private val wifiAwareManager: WifiAwareManager? by lazy {
        context.getSystemService(Context.WIFI_AWARE_SERVICE) as WifiAwareManager?
    }

    fun isNanSupported(): Boolean {
        return context.packageManager.hasSystemFeature(android.content.pm.PackageManager.FEATURE_WIFI_AWARE)
    }

    fun startPulse(encryptedPayload: ByteArray) {
        if (!isNanSupported() || wifiAwareManager == null) {
            Log.e(TAG, "Wi-Fi Aware (NAN) is not supported on this device.")
            return
        }

        if (!wifiAwareManager!!.isAvailable) {
            Log.e(TAG, "Wi-Fi Aware is currently disabled (e.g., location is off).")
            return
        }

        // 附加到 Wi-Fi Aware Session
        wifiAwareManager!!.attach(object : AttachCallback() {
            override fun onAttached(session: WifiAwareSession) {
                Log.i(TAG, "Successfully attached to Wi-Fi Aware.")
                wifiAwareSession = session
                
                // 開始廣播 (Publish)
                broadcastPayload(encryptedPayload)
                // 開始監聽 (Subscribe)
                startListening()
            }

            override fun onAttachFailed() {
                Log.e(TAG, "Failed to attach to Wi-Fi Aware.")
            }
        }, Handler(Looper.getMainLooper()))
    }

    /**
     * 無握手廣播：將 256 Bytes 的 AetherPacket 直接塞入 Service Specific Info
     */
    private fun broadcastPayload(payload: ByteArray) {
        if (payload.size > 256) {
            Log.e(TAG, "Payload exceeds 256 bytes NAN SDI limit!")
            return
        }

        val config = PublishConfig.Builder()
            .setServiceName(SERVICE_NAME)
            .setServiceSpecificInfo(payload) // 核心魔法：不建連線，直接用發現幀廣播資料
            .setTerminateNotificationEnabled(false)
            .build()

        wifiAwareSession?.publish(config, object : DiscoverySessionCallback() {
            override fun onPublishStarted(session: PublishDiscoverySession) {
                Log.i(TAG, "Aether Pulse (Publish) started successfully.")
                publishSession = session
            }
            override fun onMessageSendFailed(peerId: Int) {
                Log.e(TAG, "Failed to broadcast Aether Pulse.")
            }
        }, Handler(Looper.getMainLooper()))
    }

    /**
     * 背景監聽周圍的 Aether Pulse
     */
    private fun startListening() {
        val config = SubscribeConfig.Builder()
            .setServiceName(SERVICE_NAME)
            .build()

        wifiAwareSession?.subscribe(config, object : DiscoverySessionCallback() {
            override fun onSubscribeStarted(session: SubscribeDiscoverySession) {
                Log.i(TAG, "Aether Listener started.")
                subscribeSession = session
            }

            override fun onServiceDiscovered(
                peerHandle: PeerHandle,
                serviceSpecificInfo: ByteArray,
                matchFilter: MutableList<ByteArray>
            ) {
                Log.i(TAG, "Received Aether Pulse! Size: \${serviceSpecificInfo.size} bytes")
                
                // TODO: 透過 JNI 或 Platform Channel 將 ByteArray 交給 Rust 核心的 `routing.rs` 處理
                // handleIncomingPacketFromNan(serviceSpecificInfo)
            }
        }, Handler(Looper.getMainLooper()))
    }

    fun stopPulse() {
        publishSession?.close()
        subscribeSession?.close()
        wifiAwareSession?.close()
    }
}
