use ring::{
    aead::{self, BoundKey, OpeningKey, SealingKey, UnboundKey, NonceSequence},
    error::Unspecified,
    rand::{SecureRandom, SystemRandom},
};
use std::convert::TryInto;

const KEY_LEN: usize = 32; // 256 bits
const NONCE_LEN: usize = 12; // 96 bits

struct NonceGen {
    nonce: [u8; NONCE_LEN],
}

impl NonceGen {
    fn new(nonce: [u8; NONCE_LEN]) -> Self {
        Self { nonce }
    }
}

impl NonceSequence for NonceGen {
    fn advance(&mut self) -> Result<aead::Nonce, Unspecified> {
        aead::Nonce::try_assume_unique_for_key(&self.nonce)
    }
}

pub struct Security {
    rng: SystemRandom,
}

impl Security {
    pub fn new() -> Self {
        Self {
            rng: SystemRandom::new(),
        }
    }

    pub fn generate_key(&self) -> crate::Result<[u8; KEY_LEN]> {
        let mut key = [0u8; KEY_LEN];
        self.rng.fill(&mut key)
            .map_err(|e| crate::Error::security(format!("Failed to generate key: {:?}", e)))?;
        Ok(key)
    }

    pub fn encrypt(&self, key: &[u8], data: &[u8]) -> crate::Result<Vec<u8>> {
        let nonce = self.generate_nonce()
            .map_err(|e| crate::Error::security(format!("Failed to generate nonce: {:?}", e)))?;
        let mut nonce_gen = NonceGen::new(nonce);
        let mut key = self.create_sealing_key(key, nonce_gen)?;
        
        let mut in_out = data.to_vec();
        let aad = aead::Aad::empty();

        key.seal_in_place_append_tag(aad, &mut in_out)
            .map_err(|e| crate::Error::security(format!("Encryption failed: {:?}", e)))?;

        let mut result = nonce.to_vec();
        result.extend_from_slice(&in_out);
        Ok(result)
    }

    pub fn decrypt(&self, key: &[u8], encrypted_data: &[u8]) -> crate::Result<Vec<u8>> {
        if encrypted_data.len() < NONCE_LEN {
            return Err(crate::Error::security("Invalid encrypted data length"));
        }

        let (nonce_bytes, ciphertext) = encrypted_data.split_at(NONCE_LEN);
        let nonce = nonce_bytes.try_into()
            .map_err(|_| crate::Error::security("Invalid nonce length"))?;
        let mut nonce_gen = NonceGen::new(nonce);
        
        let mut key = self.create_opening_key(key, nonce_gen)?;
        let aad = aead::Aad::empty();

        let mut in_out = ciphertext.to_vec();
        key.open_in_place(aad, &mut in_out)
            .map_err(|e| crate::Error::security(format!("Decryption failed: {:?}", e)))?;

        Ok(in_out)
    }

    fn create_sealing_key<N: NonceSequence>(
        &self,
        key: &[u8],
        nonce_sequence: N,
    ) -> crate::Result<SealingKey<N>> {
        let unbound_key = UnboundKey::new(&aead::AES_256_GCM, key)
            .map_err(|e| crate::Error::security(format!("Invalid key: {:?}", e)))?;
        Ok(SealingKey::new(unbound_key, nonce_sequence))
    }

    fn create_opening_key<N: NonceSequence>(
        &self,
        key: &[u8],
        nonce_sequence: N,
    ) -> crate::Result<OpeningKey<N>> {
        let unbound_key = UnboundKey::new(&aead::AES_256_GCM, key)
            .map_err(|e| crate::Error::security(format!("Invalid key: {:?}", e)))?;
        Ok(OpeningKey::new(unbound_key, nonce_sequence))
    }

    fn generate_nonce(&self) -> Result<[u8; NONCE_LEN], Unspecified> {
        let mut nonce = [0u8; NONCE_LEN];
        self.rng.fill(&mut nonce)?;
        Ok(nonce)
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