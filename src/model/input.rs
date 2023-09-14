use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Query {
    pub inputs: Vec<Input>,
}

impl Query {
    pub fn new() -> Query {
        Query { inputs: Vec::new() }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Input {
    pub field: String,
    pub comparison_op: String,
    pub value: String,
}

impl Input {
    pub fn new() -> Input {
        Input {
            field: "".to_string(),
            comparison_op: "".to_string(),
            value: "".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogicalOperators {
    And(String),
    Or(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComparisonOperator {
    LessThan(String),
    GreaterThan(String),
    Equals(String),
    NotEquals(String),
    LessThanOrEquals(String),
    GreaterThanOrEquals(String),
    Default(String),
}
