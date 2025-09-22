use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "operator", content = "value")]
pub enum NumberCondition {
    Equals(f64),
    NotEquals(f64),
    GreaterThan(f64),
    LessThan(f64),
    GreaterOrEqual(f64),
    LessOrEqual(f64),
}

impl NumberCondition {
    pub fn evaluate(&self, v: &f64) -> bool {
        match self {
            NumberCondition::Equals(n) => v == n,
            NumberCondition::NotEquals(n) => v != n,
            NumberCondition::GreaterThan(n) => v > n,
            NumberCondition::LessThan(n) => v < n,
            NumberCondition::GreaterOrEqual(n) => v >= n,
            NumberCondition::LessOrEqual(n) => v <= n,
        }
    }
}
