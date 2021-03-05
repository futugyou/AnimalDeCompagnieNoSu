use crate::infrastruct::serialize::*;
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
    #[serde(default)]
    pub photoes: Vec<String>,
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
    async fn photoes(&self) -> &Vec<String> {
        &self.photoes
    }
}
