use anyhow::Result;
use domain::{CustomerId, Flow, FlowId};

#[async_trait::async_trait]
pub trait GetFlow {
    async fn execute(&self, data: GetFlowInput) -> Result<GetFlowOutput>;
}

pub struct GetFlowInput {
    pub id: FlowId,
    pub customer_id: CustomerId,
}

pub struct GetFlowOutput {
    pub flow: Flow,
}
