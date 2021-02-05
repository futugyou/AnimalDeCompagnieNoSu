use crate::repository::animalrepository::{AnimalRepository, IAnimalRepository};
use async_trait::async_trait;
use bson::doc;

use crate::model::animal::animalmodel::{
    AnimalSearchRequest, AnimalSearchResponse, AnimalUpdateRequest, AnimalUpdateResponse,
};

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
        if request.verify() {
            //TODO: add search condition
            let doc = doc! {};
            let serachresult = self.animal_repository.findmany(doc).await;
            let mut results = Vec::<AnimalSearchResponse>::new();
            for elem in serachresult {
                //TODO: convert animalentity to AnimalSearchResponse
                //let response = AnimalSearchResponse {};
                //results.push(response);
            }
            results
        } else {
            Vec::<AnimalSearchResponse>::new()
        }
    }

    async fn modfiy_animal(&self, request: AnimalUpdateRequest) -> AnimalUpdateResponse {
        todo!()
    }

    async fn delete_animal(&self, id: String) {
        todo!()
    }

    async fn find_animal_by_id(&self, id: String) -> AnimalSearchResponse {
        todo!()
    }
}
