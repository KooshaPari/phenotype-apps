//! Token wrapping, secure-storage helpers.
//!
//! The `unlock` module (QR/NFC proofs) has been archived to
//! `.archive/unlock-proof-v0/` per the 2026-04-22 scope redirect toward
//! connector-reward-gamification. Revive when the feature is prioritized again.

pub mod keychain;
#[cfg(target_vendor = "apple")]
pub use keychain::AppleKeychainStore;
#[cfg(all(target_os = "linux", not(target_vendor = "apple")))]
pub use keychain::LinuxSecretServiceStore;
pub use keychain::{default_secure_store, InMemorySecretStore, NullSecureStore};

use ring::aead::{self, BoundKey, Nonce, NonceSequence, SealingKey, UnboundKey, AES_256_GCM};
use ring::error::Unspecified;
use ring::rand::{SecureRandom, SystemRandom};
use secrecy::{ExposeSecret, SecretString};

pub trait SecureSecretStore: Send + Sync {
    fn store(&self, key: &str, value: SecretString) -> anyhow::Result<()>;
    fn load(&self, key: &str) -> anyhow::Result<Option<SecretString>>;
    fn delete(&self, key: &str) -> anyhow::Result<()>;
}

/// One-shot nonce: wraps a fixed 12-byte value for single-use sealing.
struct OneshotNonce([u8; aead::NONCE_LEN]);

impl NonceSequence for OneshotNonce {
    fn advance(&mut self) -> Result<Nonce, Unspecified> {
        Ok(Nonce::assume_unique_for_key(self.0))
    }
}

/// AES-256-GCM token wrapper.
///
/// `key` must be 32 bytes of key material encoded as a hex string in the
/// `SecretString` (matches the `API_KEY_SECRET` convention used elsewhere).
/// A fresh 12-byte nonce is generated per call via `ring::rand::SystemRandom`.
/// The nonce is prepended to the ciphertext so `decrypt` can recover it.
pub struct TokenWrap {
    /// nonce (12 bytes) || AES-256-GCM ciphertext+tag
    pub ciphertext: Vec<u8>,
    /// Convenience copy of the nonce (bytes 0..12 of `ciphertext`).
    pub nonce: Vec<u8>,
}

impl TokenWrap {
    pub fn new(key: &SecretString, plaintext: &[u8]) -> anyhow::Result<Self> {
        let raw_key = hex::decode(key.expose_secret())
            .map_err(|e| anyhow::anyhow!("TokenWrap: key must be 32-byte hex: {e}"))?;
        anyhow::ensure!(raw_key.len() == 32, "TokenWrap: key must be 32 bytes");

        let rng = SystemRandom::new();
        let mut nonce_bytes = [0u8; aead::NONCE_LEN];
        rng.fill(&mut nonce_bytes)
            .map_err(|_| anyhow::anyhow!("TokenWrap: nonce generation failed"))?;

        let unbound = UnboundKey::new(&AES_256_GCM, &raw_key)
            .map_err(|_| anyhow::anyhow!("TokenWrap: invalid key material"))?;
        let mut sealing_key = SealingKey::new(unbound, OneshotNonce(nonce_bytes));

        let mut in_out = plaintext.to_vec();
        sealing_key
            .seal_in_place_append_tag(aead::Aad::empty(), &mut in_out)
            .map_err(|_| anyhow::anyhow!("TokenWrap: AEAD seal failed"))?;

        // nonce || ciphertext+tag
        let mut combined = nonce_bytes.to_vec();
        combined.extend_from_slice(&in_out);

        Ok(Self {
            nonce: nonce_bytes.to_vec(),
            ciphertext: combined,
        })
    }
}

pub struct IntegrityDigest(pub [u8; 32]);

#[cfg(test)]
mod tests {
    use super::*;
    use secrecy::SecretString;

    fn test_key() -> SecretString {
        // 32 zero bytes as hex — for testing only.
        SecretString::new("0".repeat(64).into())
    }

    #[test]
    fn token_wrap_roundtrip_produces_ciphertext() {
        let key = test_key();
        let plaintext = b"hello, FocalPoint";
        let wrapped = TokenWrap::new(&key, plaintext).expect("seal must succeed");
        // nonce (12) + plaintext (17) + GCM tag (16) = 45
        assert_eq!(
            wrapped.ciphertext.len(),
            12 + plaintext.len() + 16,
            "ciphertext length must equal nonce + plaintext + tag"
        );
        assert_eq!(&wrapped.nonce, &wrapped.ciphertext[..12]);
    }

    #[test]
    fn token_wrap_different_calls_produce_different_nonces() {
        let key = test_key();
        let plaintext = b"nonce-uniqueness-test";
        let w1 = TokenWrap::new(&key, plaintext).unwrap();
        let w2 = TokenWrap::new(&key, plaintext).unwrap();
        // Nonces are random; collision probability is negligible.
        assert_ne!(w1.nonce, w2.nonce, "successive calls must produce unique nonces");
    }

    #[test]
    fn token_wrap_rejects_bad_key() {
        let bad_key = SecretString::new("not-hex".into());
        let result = TokenWrap::new(&bad_key, b"payload");
        assert!(result.is_err(), "non-hex key must be rejected");
    }

    #[test]
    fn token_wrap_rejects_wrong_length_key() {
        let short_key = SecretString::new("deadbeef".into()); // only 4 bytes
        let result = TokenWrap::new(&short_key, b"payload");
        assert!(result.is_err(), "short key must be rejected");
    }
}
