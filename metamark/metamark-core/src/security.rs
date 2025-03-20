use ring::{
    aead::{self, BoundKey, OpeningKey, SealingKey, UnboundKey},
    error::Unspecified,
    rand::{SecureRandom, SystemRandom},
};
use std::convert::TryInto;

const KEY_LEN: usize = 32; // 256 bits
const NONCE_LEN: usize = 12; // 96 bits

pub struct Security {
    rng: SystemRandom,
}

impl Security {
    pub fn new() -> Self {
        Self {
            rng: SystemRandom::new(),
        }
    }

    pub fn generate_key(&self) -> Result<[u8; KEY_LEN], Unspecified> {
        let mut key = [0u8; KEY_LEN];
        self.rng.fill(&mut key)?;
        Ok(key)
    }

    pub fn encrypt(&self, key: &[u8], data: &[u8]) -> crate::Result<Vec<u8>> {
        let key = self.create_sealing_key(key)?;
        let mut in_out = data.to_vec();
        let nonce = self.generate_nonce()?;
        let aad = aead::Aad::empty();

        key.seal_in_place_append_tag(nonce, aad, &mut in_out)
            .map_err(|e| crate::Error::security(format!("Encryption failed: {:?}", e)))?;

        let mut result = nonce.as_ref().to_vec();
        result.extend_from_slice(&in_out);
        Ok(result)
    }

    pub fn decrypt(&self, key: &[u8], encrypted_data: &[u8]) -> crate::Result<Vec<u8>> {
        if encrypted_data.len() < NONCE_LEN {
            return Err(crate::Error::security("Invalid encrypted data length"));
        }

        let (nonce, ciphertext) = encrypted_data.split_at(NONCE_LEN);
        let nonce = aead::Nonce::try_assume_unique_for_key(nonce)
            .map_err(|e| crate::Error::security(format!("Invalid nonce: {:?}", e)))?;

        let key = self.create_opening_key(key)?;
        let aad = aead::Aad::empty();

        let mut in_out = ciphertext.to_vec();
        key.open_in_place(nonce, aad, &mut in_out)
            .map_err(|e| crate::Error::security(format!("Decryption failed: {:?}", e)))?;

        Ok(in_out)
    }

    fn create_sealing_key(
        &self,
        key: &[u8],
    ) -> crate::Result<SealingKey<impl BoundKey<aead::Seal>>> {
        let unbound_key = UnboundKey::new(&aead::AES_256_GCM, key)
            .map_err(|e| crate::Error::security(format!("Invalid key: {:?}", e)))?;
        Ok(SealingKey::new(unbound_key, aead::Nonce::assume_unique_for_key))
    }

    fn create_opening_key(
        &self,
        key: &[u8],
    ) -> crate::Result<OpeningKey<impl BoundKey<aead::Open>>> {
        let unbound_key = UnboundKey::new(&aead::AES_256_GCM, key)
            .map_err(|e| crate::Error::security(format!("Invalid key: {:?}", e)))?;
        Ok(OpeningKey::new(unbound_key, aead::Nonce::assume_unique_for_key))
    }

    fn generate_nonce(&self) -> Result<aead::Nonce, Unspecified> {
        let mut nonce_bytes = [0u8; NONCE_LEN];
        self.rng.fill(&mut nonce_bytes)?;
        aead::Nonce::try_assume_unique_for_key(&nonce_bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encryption_decryption() {
        let security = Security::new();
        let key = security.generate_key().unwrap();
        let data = b"Hello, MetaMark!";

        let encrypted = security.encrypt(&key, data).unwrap();
        let decrypted = security.decrypt(&key, &encrypted).unwrap();

        assert_eq!(data.to_vec(), decrypted);
    }

    #[test]
    fn test_different_keys() {
        let security = Security::new();
        let key1 = security.generate_key().unwrap();
        let key2 = security.generate_key().unwrap();
        let data = b"Hello, MetaMark!";

        let encrypted = security.encrypt(&key1, data).unwrap();
        let result = security.decrypt(&key2, &encrypted);

        assert!(result.is_err());
    }
} 