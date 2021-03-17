use crate::infrastruct::serialize::*;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FileSearchResponse {
    #[serde(default)]
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

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct FileAddModel {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub ext: String,
    #[serde(default)]
    pub data: Vec<u8>,
    #[serde(with = "date_format_bson", default)]
    pub uploaddate: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FileAddResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub ext: String,
    #[serde(default)]
    pub base64src: String,
}
