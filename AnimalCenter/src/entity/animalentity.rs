use crate::infrastruct::date_format;
use crate::infrastruct::deserialize_object_id;

use chrono::{DateTime, Utc};
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
    pub fn new(
        id: String,
        name: String,
        animal_type: String,
        sub_type: String,
        birthday: DateTime<Utc>,
        idcard: String,
    ) -> Self {
        Self {
            id,
            name,
            animal_type,
            sub_type,
            birthday,
            idcard,
        }
    }

    pub fn get_collection_name() -> &'static str {
        "animal"
    }
}
