use serde::{Serialize, Deserialize};
use bson::Document;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Data {
    pub results: Vec<Document>,
}

impl Data {
    pub fn new() -> Data {
        Data {
            results: Vec::<Document>::new(),
        }
    }
}
