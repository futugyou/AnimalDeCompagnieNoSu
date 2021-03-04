use crate::{
    model::animal::animalmodel::*,
    service::animalservice::{AnimalService, IAnimalService},
};

use async_graphql::*;
use futures::{Stream, StreamExt};

pub type AnimalSchema = Schema<QueryRoot, MutationRoot, SubscriptionRoot>;

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn addanimal(&self, request: AnimalInsertRequest) -> Result<AnimalInsertResponse> {
        let service = AnimalService::new().await;
        let response = service.insert_animal(request).await?;
        Ok(response)
    }
    async fn updateanimal(&self, request: AnimalUpdateRequest) -> Result<AnimalUpdateResponse> {
        let service = AnimalService::new().await;
        let response = service.modfiy_animal(request).await?;
        Ok(response)
    }
}

pub struct QueryRoot;
#[Object]
impl QueryRoot {
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
