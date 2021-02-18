use async_trait::async_trait;

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

    async fn modfiy_animal_type(
        &self,
        _request: AnimalTypeUpdateRequest,
    ) -> AnimalTypeUpdateResponse {
        todo!()
    }
}
