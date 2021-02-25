use crate::infrastruct::{serialize::*, *};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
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
    #[serde(with = "date_format_bson", default)]
    pub birthday: Option<DateTime<Utc>>,
    #[serde(default)]
    pub idcard: String,
    #[serde(default)]
    pub avatar: String,
}

impl AnimalEntity {
    pub fn new() -> Self {
        Self {
            id: "".to_string(),
            name: "".to_string(),
            animal_type: "".to_string(),
            sub_type: "".to_string(),
            birthday: Some(getdefaultdatetime()),
            idcard: "".to_string(),
            avatar: "".to_owned(),
        }
    }

    pub fn get_collection_name() -> &'static str {
        "animal"
    }
}
