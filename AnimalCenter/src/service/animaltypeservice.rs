use async_trait::async_trait;

use crate::model::animal::BaseRequest;
use crate::{
    model::animal::animaltypemodel::{
        AnimalTypeSearchRequest, AnimalTypeSearchResponse, AnimalTypeUpdateRequest,
        AnimalTypeUpdateResponse,
    },
    repository::animaltyperepository::{AnimalTypeRepository, IAnimalTypeRepository},
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

pub struct AnimalTypeService {
    animaltype_repository: Box<dyn IAnimalTypeRepository + Send + Sync>,
}

impl AnimalTypeService {
    pub async fn new() -> Self {
        let animaltype_repository = AnimalTypeRepository::new().await;
        Self {
            animaltype_repository: Box::new(animaltype_repository),
        }
    }
}

#[async_trait]
impl IAnimalTypeService for AnimalTypeService {
    #[tracing::instrument(skip(self))]
    async fn search_animal_types(
        &self,
        request: AnimalTypeSearchRequest,
    ) -> Vec<AnimalTypeSearchResponse> {
        let doc = request.into();
        let result = self.animaltype_repository.findmany(doc).await;
        let mut reposonse = Vec::<AnimalTypeSearchResponse>::new();
        match result {
            Ok(r) => {
                for elem in r {
                    reposonse.push(elem.into());
                }
            }
            Err(e) => {
                tracing::warn!("can not fonud data : {:#?}", e);
            }
        }
        reposonse
    }
    #[tracing::instrument(skip(self))]
    async fn modfiy_animal_type(
        &self,
        request: AnimalTypeUpdateRequest,
    ) -> AnimalTypeUpdateResponse {
        let valid = request.valid();
        let mut response = AnimalTypeUpdateResponse { id: "".to_owned() };
        match valid {
            Ok(_) => {
                if request.id != "" {
                    let update_result = self.animaltype_repository.update(request.into()).await;
                    match update_result {
                        Ok(_update) => {
                            //TODO: insert when false
                        }
                        Err(error) => {
                            tracing::warn!("update error : {:#?}", error);
                        }
                    }
                } else {
                    let insert_result = self.animaltype_repository.add(request.into()).await;
                    match insert_result {
                        Ok(insert) => {
                            response.id = insert;
                        }
                        Err(error) => {
                            tracing::warn!("insert error : {:#?}", error);
                        }
                    }
                }
            }
            Err(error) => {
                tracing::warn!("request valid error : {:#?}", error);
            }
        }
        response
    }
}
