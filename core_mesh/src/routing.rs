use std::num::NonZeroUsize;
use std::sync::Arc;
use tokio::sync::Mutex;
use lru::LruCache;
use crate::protocol::AetherPacket;

pub enum GatewayType {
    None,
    Terrestrial(String), // 電信基地台 (Cell Tower)
    NonTerrestrial(String), // 低軌衛星 (SpaceX/Starlink, Globalstar)
}

// 模擬 Native 廣播介面與網路狀態
pub trait NanAdapter: Send + Sync {
    fn broadcast_nan_sdi(&self, packet: AetherPacket) -> tokio::task::JoinHandle<()>;
    fn get_local_device_id(&self) -> String;
    fn get_battery_level(&self) -> u8;
    /// 檢查是否連上電信基地台或衛星網路 (NTN)
    fn get_gateway_status(&self) -> GatewayType;
    /// 透過全球網路批次通報至緊急救難 API (消防局/醫院)
    fn dispatch_to_emergency_services(&self, packets: Vec<AetherPacket>) -> tokio::task::JoinHandle<()>;
}

pub struct MeshRouter {
    /// 本地 LRU Cache，記錄最近處理過的 Packet_ID (例如 5 分鐘內的 1000 筆)
    seen_packets: Arc<Mutex<LruCache<String, i64>>>,
    nan_adapter: Arc<dyn NanAdapter>,
}

impl MeshRouter {
    pub fn new(nan_adapter: Arc<dyn NanAdapter>) -> Self {
        // 設定容量為 1000，足以應付區域性風暴
        let cache = LruCache::new(NonZeroUsize::new(1000).unwrap());
        Self {
            seen_packets: Arc::new(Mutex::new(cache)),
            nan_adapter,
        }
    }

    /// 強制接力核心邏輯 (The Ripple Effect)
    pub async fn handle_incoming_packet(&self, packet: AetherPacket) {
        // 1. 電力檢查：大於 10% 才能轉發 (不自私原則，但也保護本機)
        if self.nan_adapter.get_battery_level() <= 10 {
            return;
        }

        // 2. 檢查 Payload 限制 (最大 256 bytes 以確保能塞入廣播幀)
        if packet.encrypted_payload.len() > 256 {
            return; // 優雅無視不合規的封包
        }

        let mut cache = self.seen_packets.lock().await;

        // 3. 智能重複刪除 (De-duplication)
        if cache.contains(&packet.message_id) {
            return; // 已處理過，優雅無視
        }

        // 4. 全球網關模式 (Global Gateway Node) 檢查
        // 偵測是否有電信基地台 (Terrestrial) 或 SpaceX 等衛星網路 (Non-Terrestrial)
        match self.nan_adapter.get_gateway_status() {
            GatewayType::None => {
                // 沒有外網，繼續 Mesh 接力 (TTL 檢查與轉發)
                if packet.ttl > 0 {
                    let mut relay_packet = packet.clone();
                    relay_packet.ttl -= 1;
                    relay_packet.last_forwarder = self.nan_adapter.get_local_device_id();
                    
                    self.nan_adapter.broadcast_nan_sdi(relay_packet);
                    cache.put(packet.message_id.clone(), chrono::Utc::now().timestamp_millis());
                }
            },
            GatewayType::Terrestrial(_) | GatewayType::NonTerrestrial(_) => {
                // 已連上全球網路 (基地台或衛星)，停止廣播，直接發射至救援中心
                self.nan_adapter.dispatch_to_emergency_services(vec![packet.clone()]);
                cache.put(packet.message_id.clone(), chrono::Utc::now().timestamp_millis());
                return;
            }
        }
    }
}
