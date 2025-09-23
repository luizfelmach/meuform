use chrono::{Duration, Utc};
use domain::{InfraError, EvaluateAnswerResult};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use protocols::Tokenizer;
use serde::{Deserialize, Serialize};

pub struct JwtTokenizer {
    secret: Vec<u8>,
    ttl_minutes: i64,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
    iat: usize,
}

impl JwtTokenizer {
    pub fn new(secret: impl Into<Vec<u8>>, ttl_minutes: i64) -> Self {
        Self {
            secret: secret.into(),
            ttl_minutes,
        }
    }
}

impl Tokenizer for JwtTokenizer {
    fn encrypt(&self, plaintext: String) -> EvaluateAnswerResult<String> {
        let now = Utc::now();
        let exp = (now + Duration::minutes(self.ttl_minutes)).timestamp() as usize;

        let claims = Claims {
            sub: plaintext,
            iat: now.timestamp() as usize,
            exp,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(&self.secret),
        )
        .map_err(|e| InfraError::EncryptionError(e.to_string()))?;

        Ok(token)
    }

    fn decrypt(&self, ciphertext: String) -> EvaluateAnswerResult<String> {
        let data = decode::<Claims>(
            &ciphertext,
            &DecodingKey::from_secret(&self.secret),
            &Validation::default(),
        )
        .map_err(|e| InfraError::DecryptionError(e.to_string()))?;

        Ok(data.claims.sub)
    }
}
