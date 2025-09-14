use anyhow::Result;
use async_trait::async_trait;
use domain::{CustomerId, Flow, Graph};
use usecases::{CreateFlow, GetFlow};

pub struct GetFlowImpl;
pub struct CreateFlowImpl;

#[async_trait]
impl GetFlow for GetFlowImpl {
    async fn execute(&self, id: &String, customer_id: &CustomerId) -> Result<Flow> {
        let flow = Flow::new(id.clone(), customer_id.clone(), Graph::default());
        Ok(flow)
    }
}

#[async_trait]
impl CreateFlow for CreateFlowImpl {
    async fn execute(&self, customer_id: &CustomerId, graph: &Graph) -> Result<Flow> {
        let flow = Flow::new("fake_id".into(), customer_id.clone(), graph.clone());
        Ok(flow)
    }
}
