use crate::infrastruct::date_format;
use crate::infrastruct::date_format_option;
use crate::infrastruct::deserialize_object_id;

use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimalSearchRequest {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    #[serde(rename = "type")]
    pub animal_type: Vec<String>,
}

impl AnimalSearchRequest {
    pub fn valid(&self) -> bool {
        true
    }
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
impl AnimalUpdateRequest {
    pub fn new() -> Self {
        Self {
            id: "".to_string(),
            name: "".to_string(),
            animal_type: "".to_string(),
            sub_type: "".to_string(),
            birthday: Utc
                .datetime_from_str("1900-01-01T01:01:01Z", "%Y-%m-%dT%H:%M:%SZ")
                .unwrap(),
        }
    }

    pub fn valid(&self) -> bool {
        //TODO: add verify
        true
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AnimalUpdateResponse {}
