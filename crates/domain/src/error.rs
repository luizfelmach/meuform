use crate::NodeId;

use chrono::{DateTime, Utc};

pub type Result<T> = std::result::Result<T, AppError>;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Erro de submissão: {0}")]
    Submission(#[from] SubmissionError),

    #[error("Erro de tela: {0}")]
    Screen(#[from] ScreenError),

    #[error("Erro no grafo: {0}")]
    Graph(#[from] GraphError),

    #[error("Erro de infraestrutura: {0}")]
    Infra(#[from] InfraError),
}

#[derive(thiserror::Error, Debug)]
pub enum SubmissionError {
    #[error("A submissão já foi concluída e não pode mais ser alterada.")]
    AlreadyCompleted,

    #[error("Não existe um passo anterior para voltar.")]
    NoPreviousNode,

    #[error("Você já está neste passo da submissão.")]
    SameNodeNavigation,

    #[error("O passo informado é inválido: {0}.")]
    InvalidNode(String),

    #[error("A resposta fornecida é inválida: {0}.")]
    InvalidAnswer(String),
}

#[derive(Debug, thiserror::Error)]
pub enum GraphError {
    #[error("O nó inicial '{0}' não existe.")]
    StartNodeMissing(NodeId),

    #[error("Aresta de '{from}' aponta para nó inexistente '{to}'.")]
    EdgeToNonExistentNode { from: NodeId, to: NodeId },

    #[error("Aresta condicional de '{from}' referencia nó inexistente '{where}'.")]
    ConditionalEdgeNodeMissing { from: NodeId, r#where: NodeId },

    #[error("Condição inválida para o nó '{0}'.")]
    InvalidCondition(NodeId),
}

#[derive(thiserror::Error, Debug)]
pub enum ScreenError {
    #[error("Campo obrigatório não pode estar vazio.")]
    RequiredField,

    #[error("Tipo de valor incompatível com o campo.")]
    TypeMismatch,

    #[error("Texto muito curto. Mínimo {min}, recebido {got}.")]
    TextTooShort { min: u32, got: u32 },

    #[error("Texto muito longo. Máximo {max}, recebido {got}.")]
    TextTooLong { max: u32, got: u32 },

    #[error("Número {got} abaixo do mínimo {min}.")]
    NumberTooSmall { min: f64, got: f64 },

    #[error("Número {got} acima do máximo {max}.")]
    NumberTooLarge { max: f64, got: f64 },

    #[error("Opção inválida: {value}.")]
    InvalidOption { value: String },

    #[error("Poucas seleções. Mínimo {min}, recebido {got}.")]
    TooFewSelections { min: u32, got: u32 },

    #[error("Muitas seleções. Máximo {max}, recebido {got}.")]
    TooManySelections { max: u32, got: u32 },

    #[error("Seleções duplicadas encontradas.")]
    DuplicateSelections,

    #[error("Data {got} anterior ao mínimo {min}.")]
    DateTooEarly {
        min: DateTime<Utc>,
        got: DateTime<Utc>,
    },

    #[error("Data {got} posterior ao máximo {max}.")]
    DateTooLate {
        max: DateTime<Utc>,
        got: DateTime<Utc>,
    },

    #[error("Campos do tipo Info não aceitam condições.")]
    InfoFieldConditionNotAllowed,

    #[error("Tipo de condição incompatível com o campo.")]
    ConditionTypeMismatch,
}

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
