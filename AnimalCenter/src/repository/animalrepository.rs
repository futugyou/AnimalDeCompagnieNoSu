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

pub struct AnimalRepository {
    collection: mongodb::Collection,
}

impl AnimalRepository {
    pub async fn new() -> Self {
        let dbcontext = DBContext {};
        let dbclient = dbcontext.get_db_context().await;
        let collection = dbclient
            .database("react-app")
            .collection(AnimalEntity::get_collection_name());
        Self { collection }
    }
}

#[async_trait]
impl IAnimalRepository for AnimalRepository {
    async fn add(&self, entity: AnimalEntity) -> String {
        let docs = doc! {
                "name": entity.name,
                "type": entity.animal_type,
                "birthday":entity.birthday,
                "sub_type":entity.sub_type,
                "idcard":entity.idcard,
        };
        let result = self.collection.insert_one(docs, None).await;
        match result {
            Ok(r) => r.inserted_id.to_string(),
            Err(e) => {
                println!("{:?}", e);
                "".to_string()
            }
        }
    }

    async fn update(&self, entity: AnimalEntity) -> Result<bool, String> {
        let filter = doc! {"_id":entity.id};
        let update = doc! {"$set" : doc!{
                "name": entity.name,
                "type": entity.animal_type,
                "birthday":entity.birthday,
                "sub_type":entity.sub_type,
                "idcard":entity.idcard,
        }};

        let result = self.collection.update_one(filter, update, None).await;
        match result {
            Ok(r) => Ok(true),
            Err(e) => Err(e.to_string()),
        }
    }

    async fn delete(&self, entity: AnimalEntity) -> bool {
        let filter = doc! {"_id":entity.id};
        let result = self.collection.delete_one(filter, None).await;
        match result {
            Ok(r) => {
                println!("ok {:?}", r);
                true
            }
            Err(e) => {
                println!("err {:?}", e);
                false
            }
        }
    }

    async fn findone<T>(&self, con: fn(t: T) -> bool) -> AnimalEntity {
        todo!()
    }

    async fn findmany<T>(&self, con: fn(t: T) -> bool) -> Vec<AnimalEntity> {
        todo!()
    }
}
