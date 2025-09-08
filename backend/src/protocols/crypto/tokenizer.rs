use anyhow::Result;

pub trait Tokenizer {
    fn encrypt(&self, plaintext: String) -> Result<String>;
    fn decrypt(&self, ciphertext: String) -> Result<String>;
}
