use std::sync::Arc;

use domain::EvaluateAnswerResult;

pub type DynHasher = Arc<dyn Hasher>;
pub type DynTokenizer = Arc<dyn Tokenizer>;

pub trait Hasher: Send + Sync {
    fn hash(&self, plaintext: String) -> EvaluateAnswerResult<String>;
    fn compare(&self, plaintext: String, digest: String) -> EvaluateAnswerResult<bool>;
}

pub trait Tokenizer: Send + Sync {
    fn encrypt(&self, plaintext: String) -> EvaluateAnswerResult<String>;
    fn decrypt(&self, ciphertext: String) -> EvaluateAnswerResult<String>;
}
