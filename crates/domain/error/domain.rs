use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("Cliente não encontrado.")]
    CustomerNotFound,

    #[error("Email já cadastrado.")]
    EmailAlreadyExists,
}
