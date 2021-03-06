use serialize::*;
use tool::*;

use bson::{doc, Document};
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
    #[serde(default)]
    pub photoes: Vec<String>,
    #[serde(with = "date_format_bson", default)]
    pub rescue_date: Option<DateTime<Utc>>,
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
            photoes: Vec::new(),
            rescue_date: Some(getutcnowwithformat()),
        }
    }

    pub fn get_collection_name() -> &'static str {
        "animal"
    }
}

impl From<&AnimalEntity> for Document {
    fn from(entity: &AnimalEntity) -> Self {
        let mut doc = doc! {
                "name": &entity.name,
                "type": &entity.animal_type,
                "sub_type": &entity.sub_type,
                "idcard": &entity.idcard,
                "avatar": &entity.avatar,
                "photoes": &entity.photoes,
        };
        if let Some(day) = &entity.birthday {
            doc.insert("birthday", day);
        }
        if let Some(day) = &entity.rescue_date {
            doc.insert("rescue_date", day);
        }
        doc
    }
}
