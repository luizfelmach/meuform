use anyhow::Result;
use domain::{Form, Submission};

pub trait GenerateUuid {
    fn generate_uuid() -> Result<String>;
}

#[async_trait::async_trait]
pub trait FindById<T> {
    async fn find_by_id(&self, id: &String) -> Result<Option<T>>;
}

#[async_trait::async_trait]
pub trait FindByEmail<T> {
    async fn find_by_email(&self, email: &String) -> Result<Option<T>>;
}

#[async_trait::async_trait]
pub trait FindBySlug<T> {
    async fn find_by_slug(&self, slug: &String) -> Result<Option<T>>;
}

#[async_trait::async_trait]
pub trait Save<T> {
    async fn save(&self, data: T) -> Result<T>;
}

#[async_trait::async_trait]
pub trait Update<T> {
    async fn update(&self, data: T) -> Result<T>;
}

#[async_trait::async_trait]
pub trait DeleteById<T> {
    async fn delete_by_id(&self, id: &String) -> Result<()>;
}

#[async_trait::async_trait]
pub trait ListForms {
    async fn list_forms(
        &self,
        customer_id: &String,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> Result<Vec<Form>>;
}

#[async_trait::async_trait]
pub trait ListSubmissions {
    async fn list_submissions(
        &self,
        form_id: &String,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> Result<Vec<Submission>>;
}
