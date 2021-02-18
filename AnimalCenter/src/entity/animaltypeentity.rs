use crate::infrastruct::serialize::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimalTypeEntity {
    #[serde(alias = "_id", default, deserialize_with = "deserialize_object_id")]
    pub id: String,
    #[serde(default)]
    pub pid: String,
    #[serde(default)]
    #[serde(rename = "type")]
    pub animal_type: String,
}
impl AnimalTypeEntity {
    pub fn new() -> Self {
        Self {
            id: "".to_owned(),
            pid: "".to_owned(),
            animal_type: "".to_owned(),
        }
    }

    pub fn get_collection_name() -> &'static str {
        "animal-types"
    }
}
