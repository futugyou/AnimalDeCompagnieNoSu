use crate::infrastruct::{serialize::*, *};
use crate::model::animal::animalmodel::custom_error::CustomError;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate; //ValidationErrornvw

pub trait BaseRequest: Validate {
    fn valid(&self) -> Result<bool, CustomError> {
        match self.validate() {
            Ok(_) => Ok(true),
            Err(_e) => {
                //Err(format!("{:#?}", e));
                Err(CustomError::new(
                    "".to_owned(),
                    "".to_owned(),
                    custom_error::CustomErrorKind::ValidateError,
                ))
            }
        }
    }
}

#[derive(Debug, Validate, Serialize, Deserialize)]
pub struct AnimalSearchRequest {
    #[serde(default)]
    #[validate(length(max = 20))]
    pub name: String,
    #[serde(default)]
    #[serde(rename = "type")]
    pub animal_type: Vec<String>,
}

impl BaseRequest for AnimalSearchRequest {}
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

impl BaseRequest for AnimalUpdateRequest {}
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
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AnimalUpdateResponse {}
