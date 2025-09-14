use anyhow::Result;
use domain::{Customer, Flow, Form, Submission};
use mongodb::{Client, Collection, options::ClientOptions};

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

        let mongo = Self {
            submission: db.collection::<Submission>("submissions"),
            form: db.collection::<Form>("forms"),
            customer: db.collection::<Customer>("customers"),
            flow: db.collection::<Flow>("flows"),
        };

        Ok(mongo)
    }
}
