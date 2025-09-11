use anyhow::Result;
use domain::{Form, Submission};

pub trait GenerateUuid: Send + Sync {
    fn generate_uuid(&self) -> Result<String>;
}

#[async_trait::async_trait]
pub trait FindById<T>: Send + Sync {
    async fn find_by_id(&self, id: &String) -> Result<Option<T>>;
}

#[async_trait::async_trait]
pub trait FindByEmail<T>: Send + Sync {
    async fn find_by_email(&self, email: &String) -> Result<Option<T>>;
}

#[async_trait::async_trait]
pub trait FindBySlug<T>: Send + Sync {
    async fn find_by_slug(&self, slug: &String) -> Result<Option<T>>;
}

#[async_trait::async_trait]
pub trait Save<T>: Send + Sync {
    async fn save(&self, data: T) -> Result<T>;
}

#[async_trait::async_trait]
pub trait Update<T>: Send + Sync {
    async fn update(&self, data: T) -> Result<T>;
}

#[async_trait::async_trait]
pub trait DeleteById<T>: Send + Sync {
    async fn delete_by_id(&self, id: &String) -> Result<()>;
}

#[async_trait::async_trait]
pub trait ListForms: Send + Sync {
    async fn list_forms(
        &self,
        customer_id: &String,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> Result<Vec<Form>>;
}

#[async_trait::async_trait]
pub trait ListSubmissions: Send + Sync {
    async fn list_submissions(
        &self,
        form_id: &String,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> Result<Vec<Submission>>;
}
