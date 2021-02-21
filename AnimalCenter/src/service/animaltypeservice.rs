use async_trait::async_trait;

use crate::{
    entity::animaltypeentity::AnimalTypeEntity, infrastruct::custom_error::CustomError,
    model::animal::BaseRequest,
};
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
    ) -> Result<AnimalTypeUpdateResponse, CustomError>;
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
    ) -> Result<AnimalTypeUpdateResponse, CustomError> {
        let valid = request.valid();
        let mut response = AnimalTypeUpdateResponse::default();
        match valid {
            Ok(_) => {
                let entity: AnimalTypeEntity = request.into();
                if entity.id != "" {
                    let update_result = self.animaltype_repository.update(entity.clone()).await;
                    match update_result {
                        Ok(update) => {
                            if update {
                                tracing::info!("update animal type ok");
                                let animaltype =
                                    self.animaltype_repository.findone(entity.id).await?;
                                response = animaltype.into();
                            } else {
                                add_new_animal_type(&self, entity, &mut response).await?;
                            }
                        }
                        Err(error) => {
                            tracing::warn!("update error : {:#?}", error);
                        }
                    }
                } else {
                    add_new_animal_type(&self, entity, &mut response).await?;
                }
            }
            Err(error) => {
                tracing::warn!("request valid error : {:#?}", error);
            }
        }
        Ok(response)
    }
}

async fn add_new_animal_type(
    svc: &AnimalTypeService,
    entity: AnimalTypeEntity,
    res: &mut AnimalTypeUpdateResponse,
) -> Result<(), CustomError> {
    *res = entity.clone().into();
    let insert_result = svc.animaltype_repository.add(entity).await?;
    res.id = insert_result;
    Ok(())
}
