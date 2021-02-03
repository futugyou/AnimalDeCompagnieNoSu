use crate::{
    entity::animalentity::AnimalEntity,
    infrastruct::context::dbcontext::{DBContext, IDbContext},
};

use async_trait::async_trait;
use bson::doc;

#[async_trait]
pub trait IAnimalRepository {
    async fn add(&self, entity: AnimalEntity) -> String;
    async fn delete(&self, entity: AnimalEntity) -> bool;
    async fn findone<T>(&self, con: fn(t: T) -> bool) -> AnimalEntity;
    async fn findmany<T>(&self, con: fn(t: T) -> bool) -> Vec<AnimalEntity>;
    async fn update(&self, entity: AnimalEntity) -> Result<bool, String>;
}

pub struct AnimalRepository {}

#[async_trait]
impl IAnimalRepository for AnimalRepository {
    async fn add(&self, entity: AnimalEntity) -> String {
        let dbcontext = DBContext {};
        let dbclient = dbcontext.get_db_context().await;
        let collection = dbclient
            .database("react-app")
            .collection(AnimalEntity::get_collection_name());
        let docs = doc! {
                "name": entity.name,
                "type": entity.animal_type,
                "birthday":entity.birthday,
                "sub_type":entity.sub_type,
                "idcard":entity.idcard,
        };
        let result = collection.insert_one(docs, None).await;
        match result {
            Ok(r) => r.inserted_id.to_string(),
            Err(e) => {
                println!("{:?}", e);
                "".to_string()
            }
        }
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
