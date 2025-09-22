#[derive(Debug, thiserror::Error)]
pub enum InfraError {
    #[error("Erro de conexão com o banco de dados")]
    DatabaseError,

    #[error("Falha ao gerar UUID: {0}")]
    UuidGenerationError(String),

    #[error("Falha ao fazer parse de UUID")]
    UuidParseError,

    #[error("Erro de rede/serviço externo: {0}")]
    ExternalServiceError(String),

    #[error("Falha ao gerar hash de senha: {0}")]
    HashError(String),

    #[error("Falha na comparação de senha: {0}")]
    CompareError(String),

    #[error("Falha ao criar token: {0}")]
    EncryptionError(String),

    #[error("Falha ao descriptografar token: {0}")]
    DecryptionError(String),
}
