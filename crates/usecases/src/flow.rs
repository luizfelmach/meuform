use anyhow::Result;
use domain::{CustomerId, Flow, Graph};
use std::sync::Arc;

pub type DynGetFlow = Arc<dyn GetFlow>;
pub type DynCreateFlow = Arc<dyn CreateFlow>;

#[async_trait::async_trait]
pub trait GetFlow: Send + Sync {
    async fn execute(&self, id: &String, customer_id: &CustomerId) -> Result<Flow>;
}

#[async_trait::async_trait]
pub trait CreateFlow: Send + Sync {
    async fn execute(&self, customer_id: &CustomerId, graph: &Graph) -> Result<Flow>;
}
