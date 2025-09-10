use anyhow::Result;

pub trait Hasher {
    fn hash(&self, plaintext: String) -> Result<String>;
    fn compare(&self, plaintext: String, digest: String) -> Result<bool>;
}

pub trait Tokenizer {
    fn encrypt(&self, plaintext: String) -> Result<String>;
    fn decrypt(&self, ciphertext: String) -> Result<String>;
}
