use infra::{
    CustomerRepositoryImpl, FlowRepositoryImpl, FormRepositoryImpl, MongoRepository,
    SubmissionRepositoryImpl,
};
use protocols::{
    DynCustomerRepository, DynFlowRepository, DynFormRepository, DynSubmissionRepository,
};
use std::sync::{Arc, LazyLock};
use tokio::runtime::Runtime;

static RUNTIME: LazyLock<Runtime> =
    LazyLock::new(|| Runtime::new().expect("failed to create Tokio runtime"));

static MONGO: LazyLock<Arc<MongoRepository>> = LazyLock::new(|| {
    let uri = "mongodb://localhost:27017".into();
    let db = "meuform".into();
    let repo = RUNTIME
        .block_on(MongoRepository::new(&uri, &db))
        .expect("failed to init mongo");
    Arc::new(repo)
});

pub fn customer() -> DynCustomerRepository {
    let repository = CustomerRepositoryImpl::new(MONGO.clone());
    return Arc::new(repository);
}

pub fn form() -> DynFormRepository {
    let repository = FormRepositoryImpl::new(MONGO.clone());
    return Arc::new(repository);
}

pub fn flow() -> DynFlowRepository {
    let repository = FlowRepositoryImpl::new(MONGO.clone());
    return Arc::new(repository);
}

pub fn submission() -> DynSubmissionRepository {
    let repository = SubmissionRepositoryImpl::new(MONGO.clone());
    return Arc::new(repository);
}
