use infra::{BcryptHasher, JwtTokenizer};
use protocols::{DynHasher, DynTokenizer};
use std::sync::Arc;

pub fn hasher() -> DynHasher {
    let hasher = BcryptHasher;
    return Arc::new(hasher);
}

pub fn tokenizer() -> DynTokenizer {
    let tokenizer = JwtTokenizer::new("secret", 60 * 8);
    return Arc::new(tokenizer);
}
