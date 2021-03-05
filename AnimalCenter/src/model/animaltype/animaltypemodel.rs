use crate::model::animal::BaseRequest;

use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimalTypeSearchRequest {
    #[serde(default)]
    #[serde(rename = "type")]
    pub animal_type: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimalTypeSearchResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub pid: String,
    #[serde(default)]
    #[serde(rename = "type")]
    pub animal_type: String,
}

#[derive(Debug, Validate, Serialize, Deserialize, Clone)]
pub struct AnimalTypeUpdateRequest {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub pid: String,
    #[serde(default)]
    #[serde(rename = "type")]
    #[validate(length(max = 20))]
    pub animal_type: String,
}

impl AnimalTypeUpdateRequest {
    pub fn new() -> Self {
        Self {
            id: "".to_owned(),
            pid: "".to_owned(),
            animal_type: "".to_owned(),
        }
    }
}

impl BaseRequest for AnimalTypeUpdateRequest {}
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct AnimalTypeUpdateResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub pid: String,
    #[serde(rename = "type")]
    pub animal_type: String,
}
