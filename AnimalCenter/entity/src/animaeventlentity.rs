use bson::Document;
use serialize::*;

use bson::doc;
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

impl From<AnimalEventEntity> for Document {
    fn from(entity: AnimalEventEntity) -> Self {
        let mut doc = doc! {
                "animalid": &entity.animalid,
                "event_type": &entity.event_type,
                "event": &entity.event,
        };
        if let Some(day) = &entity.event_time {
            doc.insert("event_time", day);
        }
        doc
    }
}
