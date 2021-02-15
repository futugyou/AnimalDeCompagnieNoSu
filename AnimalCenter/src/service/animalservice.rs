use crate::{
    entity::animalentity::AnimalEntity, infrastruct::custom_error::*,
    model::animal::animalmodel::*, repository::animalrepository::*,
};

use async_trait::async_trait;
use chrono::Utc;
use rand::Rng;

#[async_trait]
pub trait IAnimalService {
    async fn search_animals(&self, request: AnimalSearchRequest) -> Vec<AnimalSearchResponse>;
    async fn modfiy_animal(
        &self,
        request: AnimalUpdateRequest,
    ) -> Result<AnimalUpdateResponse, CustomError>;
    async fn delete_animal(&self, id: String) -> Result<(), CustomError>;
    async fn clear_fake_data(&self) -> Result<(), CustomError>;
    async fn find_animal_by_id(&self, id: String) -> AnimalSearchResponse;
}

pub struct AnimalService {
    animal_repository: Box<dyn IAnimalRepository + Send + Sync>,
}

impl AnimalService {
    pub async fn new() -> Self {
        let animal_repository = AnimalRepository::new().await;
        Self {
            animal_repository: Box::new(animal_repository),
        }
    }
}

#[async_trait]
impl IAnimalService for AnimalService {
    #[tracing::instrument(skip(self))]
    async fn search_animals(&self, request: AnimalSearchRequest) -> Vec<AnimalSearchResponse> {
        match request.valid() {
            Ok(_) => {
                let doc = request.into();
                let serach_result = self.animal_repository.findmany(doc).await;
                match serach_result {
                    Ok(search) => {
                        let mut results = Vec::<AnimalSearchResponse>::new();
                        for elem in search {
                            let response = elem.into();
                            results.push(response);
                        }
                        tracing::info!("search_animals result: {:#?}", results);
                        results
                    }
                    Err(error) => {
                        tracing::error!(
                            "search_animals call animal_repository.findmany() error: {:#?}",
                            error
                        );
                        Vec::<AnimalSearchResponse>::new()
                    }
                }
            }
            Err(message) => {
                tracing::error!("search_animals request.valid() error: {:#?}", message);
                Vec::<AnimalSearchResponse>::new()
            }
        }
    }

    #[tracing::instrument(skip(self))]
    async fn modfiy_animal(
        &self,
        request: AnimalUpdateRequest,
    ) -> Result<AnimalUpdateResponse, CustomError> {
        let results = AnimalUpdateResponse {};
        match request.valid() {
            Ok(_) => {
                let mut entity: AnimalEntity = request.into();
                if entity.id != "" {
                    let updateresult = self.animal_repository.update(entity).await;
                    match updateresult {
                        Ok(r) => {
                            //DOTO: Domain events
                            tracing::info!("call animal_repository update result: {:#?}", r);
                        }
                        Err(e) => {
                            tracing::error!("call animal_repository update error: {:#?}", e);
                            return Err(e);
                        }
                    }
                } else {
                    entity.idcard = format!(
                        "{}-{}-{:>04}",
                        &entity.animal_type,
                        Utc::now().format("%Y%m%d-%H%M%S"),
                        rand::thread_rng().gen_range(0001..9999)
                    );
                    let insertresult = self.animal_repository.add(entity).await?;
                    tracing::info!("call animal_repository add result: {:#?}", insertresult);
                }
            }
            Err(err) => {
                tracing::error!(" request.valid() error: {:#?}", err);
                return Err(err);
            }
        }
        Ok(results)
    }

    #[tracing::instrument(skip(self))]
    async fn delete_animal(&self, id: String) -> Result<(), CustomError> {
        let mut entity = AnimalEntity::new();
        entity.id = id;
        let deleteresult = self.animal_repository.delete(entity).await?;
        if deleteresult {
            //DOTO: Domain events
        }
        Ok(())
    }

    #[tracing::instrument(skip(self))]
    async fn find_animal_by_id(&self, id: String) -> AnimalSearchResponse {
        let findresult = self.animal_repository.findone(id).await;
        match findresult {
            Ok(animal) => animal.into(),
            Err(err) => {
                tracing::warn!("can not found the data , {:#?}", err);
                AnimalEntity::new().into()
            }
        }
    }

    #[tracing::instrument(skip(self))]
    async fn clear_fake_data(&self) -> Result<(), CustomError> {
        let doc = AnimalClearFakeData {}.into();
        let serachresult = self.animal_repository.findmany(doc).await?;
        for entity in serachresult {
            self.animal_repository.delete(entity).await?;
        }
        Ok(())
    }
}
