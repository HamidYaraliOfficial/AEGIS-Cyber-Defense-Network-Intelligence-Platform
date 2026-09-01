use aes_gcm::aead::{Aead, KeyInit, OsRng};
use aes_gcm::{Aes256Gcm, Nonce};
use anyhow::{anyhow, Result};
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use rand::RngCore;

use crate::storage::Database;

/// The Vault derives a 256-bit key from the user's master passphrase using
/// Argon2id, then uses AES-256-GCM for authenticated encryption. Nothing is
/// ever persisted in plain text: only the salt (stored once) and per-entry
/// nonce + ciphertext live in SQLite.
pub struct Vault<'a> {
    db: &'a Database,
    cipher: Aes256Gcm,
}

impl<'a> Vault<'a> {
    /// Derive a cipher from a master passphrase and a stored/generated salt.
    pub fn unlock(db: &'a Database, passphrase: &str, salt: &str) -> Result<Self> {
        let salt = SaltString::from_b64(salt).map_err(|e| anyhow!("invalid salt: {e}"))?;
        let argon2 = Argon2::default();
        let hash = argon2
            .hash_password(passphrase.as_bytes(), &salt)
            .map_err(|e| anyhow!("key derivation failed: {e}"))?;
        let hash_bytes = hash.hash.ok_or_else(|| anyhow!("no hash output"))?;
        let key_bytes = &hash_bytes.as_bytes()[0..32];
        let cipher = Aes256Gcm::new_from_slice(key_bytes).map_err(|e| anyhow!("{e}"))?;
        Ok(Self { db, cipher })
    }

    pub fn generate_salt() -> String {
        SaltString::generate(&mut OsRng).to_string()
    }

    pub fn put(&self, key: &str, plaintext: &str) -> Result<()> {
        let mut nonce_bytes = [0u8; 12];
        rand::thread_rng().fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);
        let ciphertext = self
            .cipher
            .encrypt(nonce, plaintext.as_bytes())
            .map_err(|e| anyhow!("encryption failed: {e}"))?;

        let repo = crate::storage::Repository::new(self.db);
        repo.vault_put(key, &hex::encode(nonce_bytes), &hex::encode(ciphertext))?;
        Ok(())
    }

    pub fn get(&self, key: &str) -> Result<Option<String>> {
        let repo = crate::storage::Repository::new(self.db);
        match repo.vault_get(key)? {
            Some((nonce_hex, ct_hex)) => {
                let nonce_bytes = hex::decode(nonce_hex)?;
                let ct_bytes = hex::decode(ct_hex)?;
                let nonce = Nonce::from_slice(&nonce_bytes);
                let plaintext = self
                    .cipher
                    .decrypt(nonce, ct_bytes.as_ref())
                    .map_err(|e| anyhow!("decryption failed (wrong passphrase?): {e}"))?;
                Ok(Some(String::from_utf8(plaintext)?))
            }
            None => Ok(None),
        }
    }

    pub fn delete(&self, key: &str) -> Result<()> {
        let repo = crate::storage::Repository::new(self.db);
        repo.vault_delete(key)
    }

    pub fn list_keys(&self) -> Result<Vec<String>> {
        let repo = crate::storage::Repository::new(self.db);
        repo.vault_list_keys()
    }
}
