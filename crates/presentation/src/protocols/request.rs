use crate::HttpError;
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use validator::Validate;

pub struct HttpRequest {
    pub body: Vec<u8>,
    pub params: HashMap<String, String>,
    pub queries: HashMap<String, String>,
    pub headers: HashMap<String, String>,
}

impl HttpRequest {
    pub fn json<T>(&self) -> Result<T, HttpError>
    where
        T: DeserializeOwned + Validate,
    {
        if self.body.is_empty() {
            return Err(HttpError::InvalidInput(
                "Empty request body, but expected JSON".into(),
            ));
        }

        let value: T = serde_json::from_slice(&self.body)
            .map_err(|e| HttpError::InvalidInput(format!("Invalid JSON body: {}", e)))?;

        value
            .validate()
            .map_err(|e| HttpError::InvalidInput(e.to_string()))?;

        Ok(value)
    }

    pub fn header(&self, key: &str) -> Result<String, HttpError> {
        self.headers
            .get(key)
            .cloned()
            .ok_or(HttpError::InvalidInput(format!("Missing header: {}", key)))
    }

    pub fn queries<T: DeserializeOwned>(&self) -> Result<T, HttpError> {
        let json = serde_json::to_string(&self.queries).unwrap();
        serde_json::from_str(&json)
            .map_err(|e| HttpError::InvalidInput(format!("Invalid query params: {}", e)))
    }

    pub fn params<T: DeserializeOwned>(&self) -> Result<T, HttpError> {
        let json = serde_json::to_string(&self.params).unwrap();
        serde_json::from_str(&json)
            .map_err(|e| HttpError::InvalidInput(format!("Invalid path params: {}", e)))
    }
}
