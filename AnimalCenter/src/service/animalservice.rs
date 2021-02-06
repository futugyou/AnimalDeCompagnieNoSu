use crate::model::animal::animalmodel::{
    AnimalSearchRequest, AnimalSearchResponse, AnimalUpdateRequest, AnimalUpdateResponse,
};
use crate::{
    entity::animalentity::AnimalEntity,
    repository::animalrepository::{AnimalRepository, IAnimalRepository},
};

use async_trait::async_trait;
use chrono::Utc;
use rand::Rng;

#[async_trait]
pub trait IAnimalService {
    async fn search_animals(&self, request: AnimalSearchRequest) -> Vec<AnimalSearchResponse>;
    async fn modfiy_animal(&self, request: AnimalUpdateRequest) -> AnimalUpdateResponse;
    async fn delete_animal(&self, id: String);
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
    async fn search_animals(&self, request: AnimalSearchRequest) -> Vec<AnimalSearchResponse> {
        if request.valid() {
            let doc = request.into();
            let serachresult = self.animal_repository.findmany(doc).await;
            let mut results = Vec::<AnimalSearchResponse>::new();
            for elem in serachresult {
                let response = elem.into();
                results.push(response);
            }
            results
        } else {
            //TODO: log
            Vec::<AnimalSearchResponse>::new()
        }
    }

    async fn modfiy_animal(&self, request: AnimalUpdateRequest) -> AnimalUpdateResponse {
        let results = AnimalUpdateResponse {};
        if request.valid() {
            let mut entity: AnimalEntity = request.into();
            if entity.id != "" {
                let updateresult = self.animal_repository.update(entity).await;
                match updateresult {
                    Ok(_re) => {
                        //DOTO: log ,Domain events
                        println!("{:?}", "update ok");
                    }
                    Err(e) => {
                        //TODO: log
                        println!("{:?}", e + ", update err");
                    }
                }
            } else {
                entity.idcard = format!(
                    "{}-{}-{:>04}",
                    &entity.animal_type,
                    Utc::now().format("%Y%m%d-%H%M%S"),
                    rand::thread_rng().gen_range(0001..9999)
                );
                let insertresult = self.animal_repository.add(entity).await;
                //TODO: log
                println!("{:?}", insertresult + ", insert result");
            }
        } else {
            //TODO: log
            println!("{:?}", "valid error");
        }
        results
    }

    async fn delete_animal(&self, id: String) {
        let mut entity = AnimalEntity::new();
        entity.id = id;
        let deleteresult = self.animal_repository.delete(entity).await;
        if deleteresult {
            //DOTO: log ,Domain events
        }
    }

    async fn find_animal_by_id(&self, id: String) -> AnimalSearchResponse {
        let findresult = self.animal_repository.findone(id).await;
        findresult.into()
    }
}
