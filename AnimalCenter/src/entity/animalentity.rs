use crate::infrastruct::date_format;
use crate::infrastruct::deserialize_object_id;

use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimalEntity {
    #[serde(alias = "_id", default, deserialize_with = "deserialize_object_id")]
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
    #[serde(default)]
    pub idcard: String,
}

impl AnimalEntity {
    pub fn new() -> Self {
        Self {
            id: "".to_string(),
            name: "".to_string(),
            animal_type: "".to_string(),
            sub_type: "".to_string(),
            birthday: Utc
                .datetime_from_str("1900-01-01T01:01:01Z", "%Y-%m-%dT%H:%M:%SZ")
                .unwrap(),
            idcard: "".to_string(),
        }
    }

    pub fn get_collection_name() -> &'static str {
        "animal"
    }
}
