use bcrypt::{DEFAULT_COST, hash, verify};
use domain::{InfraError, Result};
use protocols::Hasher;

pub struct BcryptHasher;

impl Hasher for BcryptHasher {
    fn hash(&self, plaintext: String) -> Result<String> {
        let result =
            hash(plaintext, DEFAULT_COST).map_err(|e| InfraError::HashError(e.to_string()))?;
        Ok(result)
    }

    fn compare(&self, plaintext: String, digest: String) -> Result<bool> {
        let result =
            verify(plaintext, &digest).map_err(|e| InfraError::CompareError(e.to_string()))?;
        Ok(result)
    }
}
