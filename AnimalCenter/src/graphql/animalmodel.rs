use async_graphql::*;

use chrono::{DateTime, Utc};

#[derive(Debug, InputObject)]
pub struct AnimalUpdateRequest {
    pub id: String,
    #[graphql(default)]
    pub avatar: String,
    #[graphql(default)]
    pub name: String,
    #[graphql(default)]
    pub animal_type: String,
    #[graphql(default)]
    pub sub_type: String,
    #[graphql(default)]
    pub birthday: Option<DateTime<Utc>>,
    #[graphql(default)]
    pub photoes: Vec<String>,
}

#[derive(Debug)]
pub struct AnimalUpdateResponse {
    pub id: String,
    pub name: String,
    pub animal_type: String,
    pub sub_type: String,
    pub birthday: Option<DateTime<Utc>>,
    pub idcard: String,
    pub avatar: String,
    pub photoes: Vec<String>,
}

#[Object]
impl AnimalUpdateResponse {
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
    async fn idcard(&self) -> &str {
        &self.idcard
    }
    async fn avatar(&self) -> &str {
        &self.avatar
    }
    async fn photoes(&self) -> &Vec<String> {
        &self.photoes
    }
    async fn birthday(&self) -> &Option<DateTime<Utc>> {
        &self.birthday
    }
}
#[derive(Debug, InputObject)]
pub struct AnimalSearchRequest {
    pub name: String,
    pub animal_type: Vec<String>,
}

pub struct AnimalSearchResponse {
    pub id: String,
    pub name: String,
    pub animal_type: String,
    pub sub_type: String,
    pub birthday: Option<DateTime<Utc>>,
    pub idcard: String,
    pub avatar: String,
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

#[derive(Debug, Default, InputObject)]
pub struct AnimalInsertRequest {
    pub avatar: String,
    pub name: String,
    pub animal_type: String,
    pub sub_type: String,
    pub birthday: Option<DateTime<Utc>>,
    pub photoes: Vec<String>,
}

#[derive(Default, Debug)]
pub struct AnimalInsertResponse {
    pub id: String,
    pub name: String,
    pub animal_type: String,
    pub sub_type: String,
    pub birthday: Option<DateTime<Utc>>,
    pub idcard: String,
    pub avatar: String,
    pub photoes: Vec<String>,
}

#[Object]
impl AnimalInsertResponse {
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
    async fn idcard(&self) -> &str {
        &self.idcard
    }
    async fn avatar(&self) -> &str {
        &self.avatar
    }
    async fn photoes(&self) -> &Vec<String> {
        &self.photoes
    }
    async fn birthday(&self) -> &Option<DateTime<Utc>> {
        &self.birthday
    }
}
