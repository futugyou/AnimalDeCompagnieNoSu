use crate::infrastruct::context::dbcontext::IDbContext;
use async_trait::async_trait;

use crate::viewmodel::animal::animalviewmodel::{
    AnimalSearchRequest, AnimalSearchResponse, AnimalUpdateRequest, AnimalUpdateResponse,
};

#[async_trait]
pub trait IAnimalService {
    async fn search_animals(&self, request: AnimalSearchRequest) -> Vec<AnimalSearchResponse>;
    async fn modfiy_animals(&self, request: AnimalUpdateRequest) -> AnimalUpdateResponse;
}

pub struct AnimalService {}

#[async_trait]
impl IAnimalService for AnimalService {
    async fn search_animals(&self, request: AnimalSearchRequest) -> Vec<AnimalSearchResponse> {
        todo!()
    }

    async fn modfiy_animals(&self, request: AnimalUpdateRequest) -> AnimalUpdateResponse {
        todo!()
    }
}
