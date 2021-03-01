use crate::infrastruct::{serialize::*, *};
use crate::model::animal::BaseRequest;

use async_graphql::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate; //ValidationErrornvw

#[derive(Debug, Validate, Serialize, Deserialize, InputObject)]
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
}
#[Object]
impl AnimalSearchResponse {
    async fn id(&self) -> &str {
        &self.id
    }
    async fn name(&self) -> &str {
        &self.name
    }
    async fn animal_type(&self) -> &str {
        &self.animal_type
    }
    async fn sub_type(&self) -> &str {
        &self.sub_type
    }
    async fn birthday(&self) -> &Option<DateTime<Utc>> {
        &self.birthday
    }
    async fn idcard(&self) -> &str {
        &self.idcard
    }
    async fn avatar(&self) -> &str {
        &self.avatar
    }
}

#[derive(Debug, Validate, Serialize, Deserialize, InputObject)]
pub struct AnimalUpdateRequest {
    #[serde(default)]
    pub id: String,
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
}

impl BaseRequest for AnimalUpdateRequest {}
impl AnimalUpdateRequest {
    pub fn new() -> Self {
        Self {
            id: "".to_string(),
            name: "".to_string(),
            animal_type: "".to_string(),
            sub_type: "".to_string(),
            avatar: "".to_owned(),
            birthday: Some(getdefaultdatetime()),
        }
    }
}
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct AnimalUpdateResponse {
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
}

#[Object]
impl AnimalUpdateResponse {
    async fn id(&self) -> String {
        self.id.to_string()
    }
    async fn name(&self) -> String {
        self.name.to_string()
    }
    async fn animal_type(&self) -> String {
        self.animal_type.to_string()
    }
    async fn sub_type(&self) -> String {
        self.sub_type.to_string()
    }
    async fn idcard(&self) -> String {
        self.idcard.to_string()
    }
    async fn avatar(&self) -> String {
        self.avatar.to_string()
    }
    async fn birthday(&self) -> Option<DateTime<Utc>> {
        self.birthday
    }
}
pub struct AnimalClearFakeData {}
