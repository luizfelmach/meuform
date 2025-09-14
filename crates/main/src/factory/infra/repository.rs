use infra::{
    CustomerRepositoryImpl, FlowRepositoryImpl, FormRepositoryImpl, MongoRepository,
    SubmissionRepositoryImpl,
};
use protocols::{
    DynCustomerRepository, DynFlowRepository, DynFormRepository, DynSubmissionRepository,
};
use std::sync::Arc;
use tokio::sync::OnceCell;

static MONGO: OnceCell<Arc<MongoRepository>> = OnceCell::const_new();

pub async fn init_mongo() {
    let uri = "mongodb://localhost:27017".into();
    let db = "meuform".into();

    let repo = MongoRepository::new(&uri, &db).await.unwrap();
    let _ = MONGO.set(Arc::new(repo));
}

pub fn mongo() -> Arc<MongoRepository> {
    MONGO.get().expect("mongo needs to be initialized").clone()
}

pub fn customer() -> DynCustomerRepository {
    let repository = CustomerRepositoryImpl::new(mongo());
    return Arc::new(repository);
}

pub fn form() -> DynFormRepository {
    let repository = FormRepositoryImpl::new(mongo());
    return Arc::new(repository);
}

pub fn flow() -> DynFlowRepository {
    let repository = FlowRepositoryImpl::new(mongo());
    return Arc::new(repository);
}

pub fn submission() -> DynSubmissionRepository {
    let repository = SubmissionRepositoryImpl::new(mongo());
    return Arc::new(repository);
}
