use anyhow::Result;
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

use crate::protocols::Tokenizer;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
    iat: usize,
}

pub struct JwtTokenizer {
    secret: Vec<u8>,
    ttl_minutes: i64,
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
    fn encrypt(&self, plaintext: String) -> Result<String> {
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
        )?;

        Ok(token)
    }

    fn decrypt(&self, ciphertext: String) -> Result<String> {
        let data = decode::<Claims>(
            &ciphertext,
            &DecodingKey::from_secret(&self.secret),
            &Validation::default(),
        )?;

        Ok(data.claims.sub)
    }
}
