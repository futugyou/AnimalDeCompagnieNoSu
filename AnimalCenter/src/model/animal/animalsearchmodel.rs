use crate::infrastruct::serialize::*;
use crate::model::animal::BaseRequest;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate; //ValidationErrornvw

#[derive(Debug, Validate, Serialize, Deserialize)]
pub struct AnimalSearchRequest {
    #[serde(default)]
    #[validate(length(max = 20))]
    pub name: String,
    #[serde(default)]
    #[serde(rename = "type")]
    pub animal_type: Vec<String>,
}

impl BaseRequest for AnimalSearchRequest {}
#[derive(Debug, Serialize, Deserialize)]
pub struct AnimalSearchResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    #[serde(rename = "type")]
    pub animal_type: String,
    #[serde(default)]
    pub sub_type: String,
    #[serde(with = "date_format")]
    pub birthday: Option<DateTime<Utc>>,
    #[serde(default)]
    pub idcard: String,
    #[serde(default)]
    pub avatar: String,
    #[serde(default)]
    pub photoes: Vec<String>,
}
