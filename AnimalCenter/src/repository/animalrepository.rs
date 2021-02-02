use crate::entity::animalentity::AnimalEntity;

use async_trait::async_trait;

#[async_trait]
pub trait IAnimalRepository {
    async fn add(&self, entity: AnimalEntity) -> bool;
    async fn delete(&self, entity: AnimalEntity) -> bool;
    async fn findone<T>(&self, con: fn(t: T) -> bool) -> AnimalEntity;
    async fn findmany<T>(&self, con: fn(t: T) -> bool) -> Vec<AnimalEntity>;
    async fn update(&self, entity: AnimalEntity) -> Result<bool, String>;
}

pub struct AnimalRepository {}

#[async_trait]
impl IAnimalRepository for AnimalRepository {
    async fn add(&self, entity: AnimalEntity) -> bool {
        todo!()
    }

    async fn update(&self, entity: AnimalEntity) -> Result<bool, String> {
        todo!()
    }

    async fn delete(&self, entity: AnimalEntity) -> bool {
        todo!()
    }

    async fn findone<T>(&self, con: fn(t: T) -> bool) -> AnimalEntity {
        todo!()
    }

    async fn findmany<T>(&self, con: fn(t: T) -> bool) -> Vec<AnimalEntity> {
        todo!()
    }
}
