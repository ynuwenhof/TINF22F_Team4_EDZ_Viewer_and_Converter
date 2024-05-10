use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct Package {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub sample_id: ObjectId,
    pub manufacturer_id: Option<ObjectId>,
    pub supplier_id: Option<ObjectId>,
    pub part_id: Option<ObjectId>,
    pub point_id: Option<ObjectId>,
    pub index: usize,
    pub kind: String,
    pub name: String,
    pub image: Option<PathBuf>,
}

impl Package {
    pub fn new(sample_id: ObjectId, index: usize, kind: String, name: String) -> Self {
        Self {
            id: ObjectId::new(),
            sample_id,
            manufacturer_id: None,
            supplier_id: None,
            part_id: None,
            point_id: None,
            index,
            kind,
            name,
            image: None,
        }
    }
}
