use anyhow::Result;

pub trait Hasher: Send + Sync {
    fn hash(&self, plaintext: String) -> Result<String>;
    fn compare(&self, plaintext: String, digest: String) -> Result<bool>;
}

pub trait Tokenizer: Send + Sync {
    fn encrypt(&self, plaintext: String) -> Result<String>;
    fn decrypt(&self, ciphertext: String) -> Result<String>;
}
