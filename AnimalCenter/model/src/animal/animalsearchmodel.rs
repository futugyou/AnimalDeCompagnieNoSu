use crate::PageModel;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serialize::*;
use validator::Validate;

use crate::BaseRequest;

#[derive(Debug, Clone, Validate, Deserialize)]
pub struct AnimalSearchRequest {
    #[serde(default)]
    #[validate(length(max = 20))]
    pub name: String,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_stringified_list")]
    pub animal_type: Vec<String>,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paging: Option<PageModel>,
}

impl BaseRequest for AnimalSearchRequest {}
#[derive(Debug, Serialize, Deserialize)]
pub struct AnimalSearchResponse {
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
    pub birthday: Option<DateTime<Utc>>,
    #[serde(default)]
    pub idcard: String,
    #[serde(default)]
    pub avatar: String,
    #[serde(default)]
    pub photoes: Vec<String>,
    #[serde(with = "date_format")]
    pub rescue_date: Option<DateTime<Utc>>,
}
