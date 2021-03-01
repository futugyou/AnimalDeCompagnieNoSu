use crate::{
    model::animal::animalmodel::*,
    service::animalservice::{AnimalService, IAnimalService},
};
use bson::doc;
use futures::{Stream, StreamExt};
use serde::{Deserialize, Serialize};

use async_graphql::*;

pub type AnimalSchema = Schema<QueryRoot, MutationRoot, SubscriptionRoot>;

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn addanimal(&self, request: AnimalUpdateRequest) -> Result<AnimalUpdateResponse> {
        let service = AnimalService::new().await;
        let response = service.modfiy_animal(request).await?;
        Ok(response)
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct QueryRoot {
    pub name: String,
}

#[Object]
impl QueryRoot {
    async fn name(&self) -> String {
        self.name.to_string()
    }
    async fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }
    async fn getanimals(
        &self,
        _ctx: &Context<'_>,
        request: AnimalSearchRequest,
    ) -> Vec<AnimalSearchResponse> {
        let service = AnimalService::new().await;
        let response = service.search_animals(request).await;
        response
    }
    async fn getanimal(&self, _ctx: &Context<'_>, id: String) -> AnimalSearchResponse {
        let service = AnimalService::new().await;
        let response = service.find_animal_by_id(id).await;
        response
    }
}

pub struct SubscriptionRoot;

#[Subscription]
impl SubscriptionRoot {
    async fn integers(&self, #[graphql(default = 1)] step: i32) -> impl Stream<Item = i32> {
        let mut value = 0;
        tokio::time::interval(std::time::Duration::from_secs(1)).map(move |_| {
            value += step;
            value
        })
    }
}
