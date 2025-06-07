use cloudevents::{EventBuilder, EventBuilderV10};
use entity::animalentity::AnimalEntity;
use infrastruct::context::mqcontext::{IMQContext, MQContext};
use model::animal::{animalinsertmodel::*, animalsearchmodel::*, animalupdatemodel::*, *};
use model::BaseRequest;
use repository::animalrepository::*;
use tool::custom_error::CustomError;

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
    async fn insert_animal(
        &self,
        request: AnimalInsertRequest,
    ) -> Result<AnimalInsertResponse, CustomError>;
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
                let doc = request.clone().into();
                let serach_result = self.animal_repository.findmany(doc, request.paging).await;
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
        let mut results: AnimalUpdateResponse = AnimalUpdateResponse::default();
        match request.valid() {
            Ok(_) => {
                let id = request.id.clone();
                let mut entity = Box::new(AnimalEntity::new());
                (*entity) = self.animal_repository.findone(id).await?;
                marge_entity(&mut entity, request);
                let updateresult = self.animal_repository.update(&entity).await;
                match updateresult {
                    Ok(update) => {
                        if update {
                            let mq = MQContext::new();
                            let json_message = serde_json::to_string(&entity)?;
                            let event = EventBuilderV10::new()
                                .id("0001")
                                .ty("modfiy_animal")
                                .source("http://localhost/")
                                .data("application/json", json_message)
                                .build()
                                .unwrap();
                            mq.send_message(event, "modfiy_animal", "update").await?;
                            tracing::info!("call animal_repository update result: {:#?}", update);
                            results = (*entity).into();
                        } else {
                            tracing::error!("call animal_repository add result: {:#?}", results.id);
                        }
                    }
                    Err(e) => {
                        tracing::error!("call animal_repository update error: {:#?}", e);
                        return Err(e);
                    }
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
    async fn insert_animal(
        &self,
        request: AnimalInsertRequest,
    ) -> Result<AnimalInsertResponse, CustomError> {
        match request.valid() {
            Ok(_) => {
                let mut entity: AnimalEntity = request.into();
                entity.idcard = format!(
                    "{}-{}-{:>04}",
                    &entity.animal_type,
                    Utc::now().format("%Y%m%d-%H%M%S"),
                    rand::rng().random_range(1..=9999)
                )
                .as_str()
                .to_ascii_uppercase();
                let json_message = serde_json::to_string(&entity)?;
                let mut results: AnimalInsertResponse = entity.clone().into();
                let insertresult = self.animal_repository.add(entity).await?;
                let mq = MQContext::new();
                let event = EventBuilderV10::new()
                    .id("0002")
                    .ty("insert_animal")
                    .source("http://localhost/")
                    .data("application/json", json_message)
                    .build()
                    .unwrap();
                mq.send_message(event, "modfiy_animal", "insert").await?;
                tracing::info!("call animal_repository add result: {:#?}", insertresult);
                results.id = insertresult;
                return Ok(results);
            }
            Err(err) => {
                tracing::error!("request.valid() error: {:#?}", err);
                return Err(err);
            }
        }
    }

    #[tracing::instrument(skip(self))]
    async fn delete_animal(&self, id: String) -> Result<(), CustomError> {
        let mut entity = AnimalEntity::new();
        entity.id = id;
        let json_message = serde_json::to_string(&entity)?;
        let deleteresult = self.animal_repository.delete(entity).await?;
        if deleteresult {
            let mq = MQContext::new();
            let event = EventBuilderV10::new()
                .id("0003")
                .ty("delete_animal")
                .source("http://localhost/")
                .data("application/json", json_message)
                .build()
                .unwrap();
            mq.send_message(event, "modfiy_animal", "delete").await?;
        }
        Ok(())
    }

    #[tracing::instrument(skip(self))]
    async fn clear_fake_data(&self) -> Result<(), CustomError> {
        let doc = AnimalClearFakeData {}.into();
        let serachresult = self.animal_repository.findmany(doc, None).await?;
        for entity in serachresult {
            self.animal_repository.delete(entity).await?;
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
}

fn marge_entity(animal: &mut AnimalEntity, request: AnimalUpdateRequest) -> () {
    if request.avatar != "" {
        animal.avatar = request.avatar;
    }
    if request.name != "" {
        animal.name = request.name;
    }
    if request.animal_type != "" {
        animal.animal_type = request.animal_type;
    }
    if request.sub_type != "" {
        animal.sub_type = request.sub_type;
    }
    if let Some(_a) = request.birthday {
        animal.birthday = request.birthday;
    }
    if request.photoes.len() > 0 {
        animal.photoes = request.photoes;
    }
}
