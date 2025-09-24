use crate::UseCaseResult;

use domain::{Flow, Graph};
use std::sync::Arc;

pub type DynGetFlow = Arc<dyn GetFlow>;
pub type DynCreateFlow = Arc<dyn CreateFlow>;

#[async_trait::async_trait]
pub trait GetFlow: Send + Sync {
    async fn execute(&self, id: &String) -> UseCaseResult<Flow>;
}

#[async_trait::async_trait]
pub trait CreateFlow: Send + Sync {
    async fn execute(&self, graph: &Graph) -> UseCaseResult<Flow>;
}
