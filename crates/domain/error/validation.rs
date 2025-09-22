use crate::NodeId;
use chrono::{DateTime, Utc};

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
