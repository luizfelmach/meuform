use serde::Serialize;

pub enum HttpError {
    InvalidInput(String),
    Unauthorized,
    AccessDenied,
    BadRequest(String),
    Internal(String),
}

pub enum HttpResponse {
    Ok(Vec<u8>),
    Created(Vec<u8>),
    NoContent,
}

impl HttpResponse {
    pub fn ok<T: Serialize>(data: T) -> Result<Self, HttpError> {
        match serde_json::to_vec(&data) {
            Ok(body) => Ok(HttpResponse::Ok(body)),
            Err(e) => Err(HttpError::Internal(format!("Serialization error: {}", e))),
        }
    }

    pub fn no_content() -> Result<Self, HttpError> {
        Ok(HttpResponse::NoContent)
    }

    pub fn created<T: Serialize>(data: T) -> Result<Self, HttpError> {
        match serde_json::to_vec(&data) {
            Ok(body) => Ok(HttpResponse::Created(body)),
            Err(e) => Err(HttpError::Internal(format!("Serialization error: {}", e))),
        }
    }

    pub fn error(err: HttpError) -> Result<Self, HttpError> {
        Err(err)
    }
}
