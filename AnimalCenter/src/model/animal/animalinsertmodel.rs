use crate::infrastruct::serialize::*;
use crate::model::animal::BaseRequest;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate; //ValidationErrornvw

#[derive(Debug, Default, Validate, Serialize, Deserialize)]
pub struct AnimalInsertRequest {
    #[serde(default)]
    pub avatar: String,
    #[serde(default)]
    #[validate(length(
        min = 2,
        max = 20,
        message = "`name` length must big than {min} and less than {max}"
    ))]
    pub name: String,
    #[serde(default)]
    #[serde(rename = "type")]
    #[validate(length(
        min = 2,
        max = 20,
        message = "`type` length must big than {min} and less than {max}"
    ))]
    pub animal_type: String,
    #[serde(default)]
    #[validate(length(
        min = 2,
        max = 20,
        message = "`sub_type` length must big than {min} and less than {max}"
    ))]
    pub sub_type: String,
    #[serde(with = "date_format", default)]
    pub birthday: Option<DateTime<Utc>>,
    #[serde(default)]
    pub photoes: Vec<String>,
}

impl BaseRequest for AnimalInsertRequest {}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct AnimalInsertResponse {
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
}
