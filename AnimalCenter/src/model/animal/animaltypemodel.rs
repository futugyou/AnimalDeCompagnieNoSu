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

#[derive(Debug, Validate, Serialize, Deserialize)]
pub struct AnimalTypeUpdateRequest {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    #[serde(rename = "type")]
    #[validate(length(max = 20))]
    pub animal_type: String,
}

impl BaseRequest for AnimalTypeUpdateRequest {}
#[derive(Debug, Serialize, Deserialize)]
pub struct AnimalTypeUpdateResponse {}
