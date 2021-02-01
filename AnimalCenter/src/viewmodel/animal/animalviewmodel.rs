use crate::infrastruct::date_format;
use crate::infrastruct::date_format_option;
use crate::infrastruct::deserialize_object_id;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimalSearchRequest {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    #[serde(rename = "type")]
    pub animal_type: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimalSearchResponse {
    #[serde(alias = "_id", default, deserialize_with = "deserialize_object_id")]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    #[serde(rename = "type")]
    pub animal_type: String,
    #[serde(default)]
    pub sub_type: String,
    #[serde(with = "date_format_option", default)]
    pub birthday: Option<DateTime<Utc>>,
    #[serde(default)]
    pub idcard: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimalUpdateRequest {
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
    pub birthday: DateTime<Utc>,
}
