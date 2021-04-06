use serialize::*;
use tool::*;

use bson::{doc, Document};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct AnimalEventEntity {
    #[serde(alias = "_id", default, deserialize_with = "deserialize_object_id")]
    pub id: String,
    #[serde(default)]
    pub animalid: String,
    #[serde(default)]
    pub event_type: String,
    #[serde(default)]
    pub event: String,
    #[serde(with = "date_format_bson", default)]
    pub event_time: Option<DateTime<Utc>>,
}

impl AnimalEventEntity {
    pub fn new() -> Self {
        AnimalEventEntity::default()
    }

    pub fn get_collection_name() -> &'static str {
        "animal-event"
    }
}
