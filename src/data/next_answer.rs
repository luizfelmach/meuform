use crate::{
    core::{Answer, AnswerType, Condition, Edge},
    protocols::{AnswerRepository, GraphRepository},
    usecase::{NextAnswer, NextAnswerInput, NextAnswerOutput},
};
use anyhow::{Ok, Result, anyhow, bail};

pub struct NextAnswerUseCase<G, A> {
    pub graph: G,
    pub answer: A,
}

impl<G, A> NextAnswerUseCase<G, A>
where
    G: GraphRepository,
    A: AnswerRepository,
{
    pub fn new(graph: G, answer: A) -> Self {
        Self { graph, answer }
    }
}

impl<G, A> NextAnswer for NextAnswerUseCase<G, A>
where
    G: GraphRepository,
    A: AnswerRepository,
{
    async fn next(&self, data: NextAnswerInput) -> anyhow::Result<NextAnswerOutput> {
        let mut answer = self
            .answer
            .get_by_id(&data.answer_id)
            .await?
            .ok_or_else(|| anyhow!("Answer not found"))?;

        if answer.completed {
            bail!("Answer is completed")
        }

        let graph = self
            .graph
            .get_by_id(&answer.graph_id)
            .await?
            .ok_or_else(|| anyhow!("Graph not found"))?;

        if graph.end.contains(&answer.current_node) {
            return Ok(NextAnswerOutput { completed: true });
        }

        let edges = graph
            .edges
            .get(&answer.current_node)
            .ok_or_else(|| anyhow!("Could not load edges"))?;

        let mut target = None;

        for edge in edges {
            match edge {
                Edge::Unconditional { to } => {
                    target = Some(*to);
                    break;
                }
                Edge::Conditional { to, condition } => {
                    if evaluate_condition(condition, &answer)? {
                        target = Some(*to);
                        break;
                    }
                }
            }
        }

        match target {
            None => Ok(NextAnswerOutput { completed: false }),
            Some(node) => {
                answer.current_node = node;
                let _ = self.answer.update(answer);
                Ok(NextAnswerOutput { completed: false })
            }
        }
    }
}

fn evaluate_condition(condition: &Condition, answer: &Answer) -> Result<bool> {
    match condition {
        // === TEXT ===
        Condition::TextEquals(id, expected) => match answer.responses.get(id) {
            Some(AnswerType::Text(resp)) => Ok(resp == expected),
            Some(AnswerType::TextArea(resp)) => Ok(resp == expected),
            Some(AnswerType::Radio(resp)) => Ok(resp == expected),
            _ => Ok(false),
        },
        Condition::TextNotEquals(id, expected) => match answer.responses.get(id) {
            Some(AnswerType::Text(resp)) => Ok(resp != expected),
            Some(AnswerType::TextArea(resp)) => Ok(resp != expected),
            Some(AnswerType::Radio(resp)) => Ok(resp != expected),
            _ => Ok(false),
        },
        Condition::TextIn(id, values) => match answer.responses.get(id) {
            Some(AnswerType::Text(resp)) => Ok(values.contains(resp)),
            Some(AnswerType::TextArea(resp)) => Ok(values.contains(resp)),
            Some(AnswerType::Radio(resp)) => Ok(values.contains(resp)),
            Some(AnswerType::Checkbox(resp_values)) => {
                Ok(resp_values.iter().any(|v| values.contains(v)))
            }
            _ => Ok(false),
        },
        Condition::TextNotIn(id, values) => match answer.responses.get(id) {
            Some(AnswerType::Text(resp)) => Ok(!values.contains(resp)),
            Some(AnswerType::TextArea(resp)) => Ok(!values.contains(resp)),
            Some(AnswerType::Radio(resp)) => Ok(!values.contains(resp)),
            Some(AnswerType::Checkbox(resp_values)) => {
                Ok(resp_values.iter().all(|v| !values.contains(v)))
            }
            _ => Ok(false),
        },

        // === NUMBERS ===
        Condition::NumberEquals(id, expected) => match answer.responses.get(id) {
            Some(AnswerType::Number(resp)) => Ok((resp - expected).abs() < f64::EPSILON),
            _ => Ok(false),
        },
        Condition::NumberNotEquals(id, expected) => match answer.responses.get(id) {
            Some(AnswerType::Number(resp)) => Ok((resp - expected).abs() > f64::EPSILON),
            _ => Ok(false),
        },
        Condition::NumberGreaterThan(id, expected) => match answer.responses.get(id) {
            Some(AnswerType::Number(resp)) => Ok(resp > expected),
            _ => Ok(false),
        },
        Condition::NumberLessThan(id, expected) => match answer.responses.get(id) {
            Some(AnswerType::Number(resp)) => Ok(resp < expected),
            _ => Ok(false),
        },

        // === BOOLEAN ===
        Condition::BooleanEquals(id, expected) => match answer.responses.get(id) {
            Some(AnswerType::Boolean(resp)) => Ok(resp == expected),
            _ => Ok(false),
        },

        // === DATE ===
        Condition::DateEquals(id, expected) => match answer.responses.get(id) {
            Some(AnswerType::Date(resp)) => Ok(resp == expected),
            _ => Ok(false),
        },
        Condition::DateNotEquals(id, expected) => match answer.responses.get(id) {
            Some(AnswerType::Date(resp)) => Ok(resp != expected),
            _ => Ok(false),
        },
        Condition::DateBefore(id, expected) => match answer.responses.get(id) {
            Some(AnswerType::Date(resp)) => Ok(resp < expected),
            _ => Ok(false),
        },
        Condition::DateAfter(id, expected) => match answer.responses.get(id) {
            Some(AnswerType::Date(resp)) => Ok(resp > expected),
            _ => Ok(false),
        },
    }
}
