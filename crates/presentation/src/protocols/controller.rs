use crate::{HttpError, HttpRequest, HttpResponse};

use std::sync::Arc;

#[async_trait::async_trait]
pub trait Controller: Send + Sync {
    async fn handle(&self, request: HttpRequest) -> Result<HttpResponse, HttpError>;
}

pub type DynController = Arc<dyn Controller>;

#[macro_export]
macro_rules! controller {
    ($handler:expr) => {
        Arc::new($handler) as DynController
    };
}
