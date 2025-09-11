use anyhow::Result;
use domain::{Customer, Flow, Form, Submission};
use futures_util::stream::TryStreamExt;
use mongodb::bson::{doc, oid::ObjectId};
use mongodb::{Client, Collection, options::ClientOptions};
use protocols::{DeleteById, FindByEmail, FindById, FindBySlug};
use protocols::{GenerateUuid, ListForms, ListSubmissions, Save, Update};

pub struct MongoRepository {
    pub submission: Collection<Submission>,
    pub form: Collection<Form>,
    pub customer: Collection<Customer>,
    pub flow: Collection<Flow>,
}

impl MongoRepository {
    pub async fn new(uri: &String, db: &String) -> Result<Self> {
        let options = ClientOptions::parse(uri).await?;
        let client = Client::with_options(options)?;
        let db = client.database(db);

        Ok(Self {
            submission: db.collection::<Submission>("submissions"),
            form: db.collection::<Form>("forms"),
            customer: db.collection::<Customer>("customers"),
            flow: db.collection::<Flow>("flows"),
        })
    }
}

impl GenerateUuid for MongoRepository {
    fn generate_uuid(&self) -> Result<String> {
        Ok(ObjectId::new().to_string())
    }
}

#[async_trait::async_trait]
impl FindById<Customer> for MongoRepository {
    async fn find_by_id(&self, id: &String) -> Result<Option<Customer>> {
        let oid = ObjectId::parse_str(id)?;
        let filter = doc! { "_id": oid };
        let data = self.customer.find_one(filter).await?;
        Ok(data)
    }
}

#[async_trait::async_trait]
impl FindById<Form> for MongoRepository {
    async fn find_by_id(&self, id: &String) -> Result<Option<Form>> {
        let oid = ObjectId::parse_str(id)?;
        let filter = doc! { "_id": oid };
        let data = self.form.find_one(filter).await?;
        Ok(data)
    }
}

#[async_trait::async_trait]
impl FindById<Submission> for MongoRepository {
    async fn find_by_id(&self, id: &String) -> Result<Option<Submission>> {
        let oid = ObjectId::parse_str(id)?;
        let filter = doc! { "_id": oid };
        let data = self.submission.find_one(filter).await?;
        Ok(data)
    }
}

#[async_trait::async_trait]
impl FindById<Flow> for MongoRepository {
    async fn find_by_id(&self, id: &String) -> Result<Option<Flow>> {
        let oid = ObjectId::parse_str(id)?;
        let filter = doc! { "_id": oid };
        let data = self.flow.find_one(filter).await?;
        Ok(data)
    }
}

#[async_trait::async_trait]
impl FindByEmail<Customer> for MongoRepository {
    async fn find_by_email(&self, email: &String) -> Result<Option<Customer>> {
        let filter = doc! { "email": email };
        let data = self.customer.find_one(filter).await?;
        Ok(data)
    }
}

#[async_trait::async_trait]
impl FindBySlug<Form> for MongoRepository {
    async fn find_by_slug(&self, slug: &String) -> Result<Option<Form>> {
        let filter = doc! { "slug": slug };
        let data = self.form.find_one(filter).await?;
        Ok(data)
    }
}

#[async_trait::async_trait]
impl Save<Customer> for MongoRepository {
    async fn save(&self, data: Customer) -> Result<Customer> {
        let _ = self.customer.insert_one(&data).await?;
        Ok(data)
    }
}

#[async_trait::async_trait]
impl Save<Form> for MongoRepository {
    async fn save(&self, data: Form) -> Result<Form> {
        let _ = self.form.insert_one(&data).await?;
        Ok(data)
    }
}

#[async_trait::async_trait]
impl Save<Flow> for MongoRepository {
    async fn save(&self, data: Flow) -> Result<Flow> {
        let _ = self.flow.insert_one(&data).await?;
        Ok(data)
    }
}

#[async_trait::async_trait]
impl Save<Submission> for MongoRepository {
    async fn save(&self, data: Submission) -> Result<Submission> {
        let _ = self.submission.insert_one(&data).await?;
        Ok(data)
    }
}

#[async_trait::async_trait]
impl Update<Customer> for MongoRepository {
    async fn update(&self, data: Customer) -> Result<Customer> {
        let oid = ObjectId::parse_str(data.id.clone())?;
        let filter = doc! { "_id": &oid };
        self.customer.replace_one(filter, &data).await?;
        Ok(data)
    }
}

#[async_trait::async_trait]
impl Update<Form> for MongoRepository {
    async fn update(&self, data: Form) -> Result<Form> {
        let oid = ObjectId::parse_str(data.id.clone())?;
        let filter = doc! { "_id": &oid };
        self.form.replace_one(filter, &data).await?;
        Ok(data)
    }
}

#[async_trait::async_trait]
impl Update<Submission> for MongoRepository {
    async fn update(&self, data: Submission) -> Result<Submission> {
        let oid = ObjectId::parse_str(data.id.clone())?;
        let filter = doc! { "_id": &oid };
        self.submission.replace_one(filter, &data).await?;
        Ok(data)
    }
}

#[async_trait::async_trait]
impl DeleteById<Customer> for MongoRepository {
    async fn delete_by_id(&self, id: &String) -> Result<()> {
        let oid = ObjectId::parse_str(id)?;
        let filter = doc! { "_id": oid };
        self.customer.delete_one(filter).await?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl DeleteById<Form> for MongoRepository {
    async fn delete_by_id(&self, id: &String) -> Result<()> {
        let oid = ObjectId::parse_str(id)?;
        let filter = doc! { "_id": oid };
        self.form.delete_one(filter).await?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl DeleteById<Submission> for MongoRepository {
    async fn delete_by_id(&self, id: &String) -> Result<()> {
        let oid = ObjectId::parse_str(id)?;
        let filter = doc! { "_id": oid };
        self.submission.delete_one(filter).await?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl DeleteById<Flow> for MongoRepository {
    async fn delete_by_id(&self, id: &String) -> Result<()> {
        let oid = ObjectId::parse_str(id)?;
        let filter = doc! { "_id": oid };
        self.flow.delete_one(filter).await?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl ListForms for MongoRepository {
    async fn list_forms(
        &self,
        customer_id: &String,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> Result<Vec<Form>> {
        let filter = doc! { "customer_id": customer_id };
        let mut cursor = self
            .form
            .find(filter)
            .limit(limit.unwrap_or(100) as i64)
            .skip(offset.unwrap_or(0) as u64)
            .await?;

        let mut result = Vec::new();

        while let Some(data) = cursor.try_next().await? {
            result.push(data);
        }

        Ok(result)
    }
}

#[async_trait::async_trait]
impl ListSubmissions for MongoRepository {
    async fn list_submissions(
        &self,
        form_id: &String,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> Result<Vec<Submission>> {
        let filter = doc! { "form_id": form_id };
        let mut cursor = self
            .submission
            .find(filter)
            .limit(limit.unwrap_or(100) as i64)
            .skip(offset.unwrap_or(0) as u64)
            .await?;

        let mut result = Vec::new();

        while let Some(data) = cursor.try_next().await? {
            result.push(data);
        }

        Ok(result)
    }
}
