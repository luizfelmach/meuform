use crate::core::{Answer, Customer, Form, Graph};
use anyhow::Result;
use mongodb::{Client, Collection, options::ClientOptions};

pub struct MongoDb {
    pub answer: Collection<Answer>,
    pub form: Collection<Form>,
    pub customer: Collection<Customer>,
    pub graph: Collection<Graph>,
}

impl MongoDb {
    pub async fn new(uri: &String, db: &String) -> Result<Self> {
        let options = ClientOptions::parse(uri).await?;
        let client = Client::with_options(options)?;
        let db = client.database(db);

        Ok(Self {
            answer: db.collection::<Answer>("answers"),
            form: db.collection::<Form>("forms"),
            customer: db.collection::<Customer>("customers"),
            graph: db.collection::<Graph>("graphs"),
        })
    }
}
