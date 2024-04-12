use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Company {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    #[serde(flatten)]
    pub map: HashMap<String, String>,
}

impl Company {
    pub fn new(map: HashMap<String, String>) -> Self {
        Self {
            id: ObjectId::new(),
            map,
        }
    }
}
