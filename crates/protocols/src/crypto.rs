use std::sync::Arc;

use domain::Result;

pub type DynHasher = Arc<dyn Hasher>;
pub type DynTokenizer = Arc<dyn Tokenizer>;

pub trait Hasher: Send + Sync {
    fn hash(&self, plaintext: String) -> Result<String>;
    fn compare(&self, plaintext: String, digest: String) -> Result<bool>;
}

pub trait Tokenizer: Send + Sync {
    fn encrypt(&self, plaintext: String) -> Result<String>;
    fn decrypt(&self, ciphertext: String) -> Result<String>;
}
