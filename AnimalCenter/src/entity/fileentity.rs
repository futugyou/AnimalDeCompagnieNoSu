use crate::infrastruct::serialize::*;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct FileEntity {
    #[serde(alias = "_id", default, deserialize_with = "deserialize_object_id")]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub ext: String,
    #[serde(default)]
    pub base64src: String,
    #[serde(with = "naive_date_format_bson", default)]
    pub uploaddate: Option<NaiveDateTime>,
}

impl FileEntity {
    pub fn get_collection_name() -> &'static str {
        "fileupload"
    }
}
