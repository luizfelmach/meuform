use anyhow::{Result, anyhow};
use bcrypt::{DEFAULT_COST, hash, verify};

use crate::protocols::Hasher;

pub struct BcryptHasher;

impl Hasher for BcryptHasher {
    fn hash(&self, plaintext: String) -> Result<String> {
        hash(plaintext, DEFAULT_COST).map_err(|e| anyhow!("Failed to hash password: {}", e))
    }

    fn compare(&self, plaintext: String, digest: String) -> Result<bool> {
        verify(plaintext, &digest).map_err(|e| anyhow!("Failed to verify password: {}", e))
    }
}
