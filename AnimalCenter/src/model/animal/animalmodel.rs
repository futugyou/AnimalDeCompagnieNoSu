use crate::infrastruct::{serialize::*, *};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate; //ValidationErrornvw

#[derive(Debug, Validate, Serialize, Deserialize)]
pub struct AnimalSearchRequest {
    #[serde(default)]
    #[validate(length(max = 20))]
    pub name: String,
    #[serde(default)]
    #[serde(rename = "type")]
    pub animal_type: Vec<String>,
}

impl AnimalSearchRequest {
    pub fn valid(&self) -> Result<bool, String> {
        match self.validate() {
            Ok(_) => Ok(true),
            Err(e) => Err(format!("{:#?}", e)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimalSearchResponse {
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
    pub birthday: Option<DateTime<Utc>>,
    #[serde(default)]
    pub idcard: String,
}

#[derive(Debug, Validate, Serialize, Deserialize)]
pub struct AnimalUpdateRequest {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    #[validate(length(min = 2, max = 20))]
    pub name: String,
    #[serde(default)]
    #[serde(rename = "type")]
    #[validate(length(min = 2, max = 20))]
    pub animal_type: String,
    #[serde(default)]
    #[validate(length(min = 2, max = 20))]
    pub sub_type: String,
    #[serde(with = "date_format", default)]
    pub birthday: Option<DateTime<Utc>>,
}
impl AnimalUpdateRequest {
    pub fn new() -> Self {
        Self {
            id: "".to_string(),
            name: "".to_string(),
            animal_type: "".to_string(),
            sub_type: "".to_string(),
            birthday: Some(getdefaultdatetime()),
        }
    }

    pub fn valid(&self) -> Result<bool, String> {
        match self.validate() {
            Ok(_) => Ok(true),
            Err(e) => Err(format!("{:#?}", e)),
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AnimalUpdateResponse {}
