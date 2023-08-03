use serde::{Serialize, Deserialize};

#[cfg(feature = "ssr")]
use mongodb::bson::Document;

#[cfg(feature = "ssr")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Data {
    pub results: Vec<Document>,
}
