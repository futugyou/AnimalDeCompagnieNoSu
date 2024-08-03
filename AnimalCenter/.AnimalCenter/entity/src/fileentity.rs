use serialize::*;

use chrono::{DateTime, Utc};
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
    #[serde(with = "date_format_bson", default)]
    pub uploaddate: Option<DateTime<Utc>>,
}

impl FileEntity {
    pub fn get_collection_name() -> &'static str {
        "fileupload"
    }
}
