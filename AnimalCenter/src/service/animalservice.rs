use crate::model::animal::animalmodel::{
    AnimalSearchRequest, AnimalSearchResponse, AnimalUpdateRequest, AnimalUpdateResponse,
};
use crate::{
    entity::animalentity::AnimalEntity,
    repository::animalrepository::{AnimalRepository, IAnimalRepository},
};

use async_trait::async_trait;

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
            //TODO: add search condition
            let entity: AnimalEntity = AnimalEntity::new();
            let updateresult = self.animal_repository.update(entity).await;
            match updateresult {
                Ok(_re) => {
                    //DOTO: log ,Domain events
                    println!("{:?}", "do something");
                }
                Err(e) => {
                    //TODO: log
                    println!("{:?}", e + "do something");
                }
            }
        } else {
            //TODO: log
            println!("{:?}", "do something");
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
        //TODO: convert Animalentity to AnimalSearchResponse
        todo!()
    }
}
