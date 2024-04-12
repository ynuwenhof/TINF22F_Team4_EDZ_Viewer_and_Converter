use bson::oid::ObjectId;
use bson::serde_helpers::time_0_3_offsetdatetime_as_bson_datetime as bson_datetime;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Serialize, Deserialize)]
pub struct Sample {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub hash: String,
    pub name: String,
    pub size: usize,
    pub packages: usize,
    #[serde(with = "bson_datetime")]
    pub created: OffsetDateTime,
    #[serde(with = "bson_datetime")]
    pub completed: OffsetDateTime,
    #[serde(with = "bson_datetime")]
    pub expires: OffsetDateTime,
}
