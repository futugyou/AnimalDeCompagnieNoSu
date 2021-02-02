use crate::infrastruct::deserialize_object_id;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimalTypeSearchRequest {
    #[serde(default)]
    #[serde(rename = "type")]
    pub animal_type: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimalTypeSearchResponse {
    #[serde(alias = "_id", default, deserialize_with = "deserialize_object_id")]
    pub id: String,
    #[serde(default)]
    pub pid: String,
    #[serde(default)]
    #[serde(rename = "type")]
    pub animal_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimalTypeUpdateRequest {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    #[serde(rename = "type")]
    pub animal_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimalTypeUpdateResponse {}
