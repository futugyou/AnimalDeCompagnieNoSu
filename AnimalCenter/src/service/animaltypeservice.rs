use async_trait::async_trait;

use crate::model::animal::animaltypemodel::{
    AnimalTypeSearchRequest, AnimalTypeSearchResponse, AnimalTypeUpdateRequest,
    AnimalTypeUpdateResponse,
};

#[async_trait]
pub trait IAnimalTypeService {
    async fn search_animal_types(
        &self,
        request: AnimalTypeSearchRequest,
    ) -> Vec<AnimalTypeSearchResponse>;
    async fn modfiy_animal_type(
        &self,
        request: AnimalTypeUpdateRequest,
    ) -> AnimalTypeUpdateResponse;
}

pub struct AnimalTypeService {}

#[async_trait]
impl IAnimalTypeService for AnimalTypeService {
    async fn search_animal_types(
        &self,
        _request: AnimalTypeSearchRequest,
    ) -> Vec<AnimalTypeSearchResponse> {
        todo!()
    }

    async fn modfiy_animal_type(
        &self,
        _request: AnimalTypeUpdateRequest,
    ) -> AnimalTypeUpdateResponse {
        todo!()
    }
}
