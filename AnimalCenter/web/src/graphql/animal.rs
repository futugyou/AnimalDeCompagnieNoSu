use async_graphql::*;
use futures::{Stream, StreamExt};
use model::animal::animalmodel::*;
use service::animalservice::{AnimalService, IAnimalService};
use std::time::Duration;

pub type AnimalSchema = Schema<QueryRoot, MutationRoot, SubscriptionRoot>;

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn addanimal(&self, request: AnimalInsertRequest) -> Result<AnimalInsertResponse> {
        let service = AnimalService::new().await;
        let response = service.insert_animal(request.into()).await?;
        Ok(response.into())
    }
    async fn updateanimal(&self, request: AnimalUpdateRequest) -> Result<AnimalUpdateResponse> {
        let service = AnimalService::new().await;
        let response = service.modfiy_animal(request.into()).await?;
        Ok(response.into())
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
        let response = service.search_animals(request.into()).await;
        response.into_iter().map(|x| x.into()).collect()
    }
    async fn getanimal(&self, _ctx: &Context<'_>, id: String) -> AnimalSearchResponse {
        let service = AnimalService::new().await;
        let response = service.find_animal_by_id(id).await;
        response.into()
    }
}

pub struct SubscriptionRoot;

#[Subscription]
impl SubscriptionRoot {
    async fn integers(&self, #[graphql(default = 1)] step: i32) -> impl Stream<Item = i32> {
        let mut value = 0;
        tokio_stream::wrappers::IntervalStream::new(tokio::time::interval(Duration::from_secs(1)))
            .map(move |_| {
                value += step;
                value
            })
    }
}
