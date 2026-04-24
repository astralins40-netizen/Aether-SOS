use chacha20poly1305::{
    aead::{Aead, KeyInit, OsRng},
    ChaCha20Poly1305, Key, Nonce,
};
use rand::RngCore;

/// Aether 加密模組：使用 ChaCha20-Poly1305 確保端到端加密與完整性
/// 這是目前最適合行動端與嵌入式設備的高效能軍規加密演算法
pub struct AetherCrypto {
    cipher: ChaCha20Poly1305,
}

impl AetherCrypto {
    /// 初始化加密模組 (使用共享的密鑰，實務上可透過 Diffie-Hellman 交換)
    pub fn new(key_bytes: &[u8; 32]) -> Self {
        let key = Key::from_slice(key_bytes);
        let cipher = ChaCha20Poly1305::new(key);
        Self { cipher }
    }

    /// 產生隨機金鑰 (用於測試或首次初始化)
    pub fn generate_key() -> [u8; 32] {
        let mut key = [0u8; 32];
        OsRng.fill_bytes(&mut key);
        key
    }

    /// 加密 Payload，並回傳 [Nonce (12 bytes) + Ciphertext]
    pub fn encrypt_payload(&self, plaintext: &[u8]) -> Result<Vec<u8>, String> {
        // 產生 12 bytes 的隨機 Nonce
        let mut nonce_bytes = [0u8; 12];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        // 加密
        let ciphertext = self.cipher.encrypt(nonce, plaintext)
            .map_err(|e| format!("Encryption failed: {}", e))?;

        // 將 Nonce 附加在密文前方，解密時需要
        let mut result = Vec::with_capacity(12 + ciphertext.len());
        result.extend_from_slice(&nonce_bytes);
        result.extend_from_slice(&ciphertext);

        Ok(result)
    }

    /// 解密 Payload，輸入必須是 [Nonce (12 bytes) + Ciphertext]
    pub fn decrypt_payload(&self, encrypted_data: &[u8]) -> Result<Vec<u8>, String> {
        if encrypted_data.len() < 12 {
            return Err("Invalid payload size, missing Nonce".to_string());
        }

        let nonce = Nonce::from_slice(&encrypted_data[0..12]);
        let ciphertext = &encrypted_data[12..];

        // 解密並驗證完整性 (Poly1305 認證)
        self.cipher.decrypt(nonce, ciphertext)
            .map_err(|_| "Decryption failed or payload tampered".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encryption_decryption() {
        let key = AetherCrypto::generate_key();
        let crypto = AetherCrypto::new(&key);

        // 模擬包含了受困者血氧與具體位置的敏感 Payload
        let secret_message = b"HeartRate:85,Oxygen:92,Status:Trapped_under_rubble";

        let encrypted = crypto.encrypt_payload(secret_message).unwrap();
        assert_ne!(encrypted, secret_message);

        let decrypted = crypto.decrypt_payload(&encrypted).unwrap();
        assert_eq!(decrypted, secret_message);
    }
}
