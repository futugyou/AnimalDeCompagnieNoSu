use crate::{
    entity::animalentity::AnimalEntity,
    infrastruct::context::dbcontext::{DBContext, IDbContext},
};
use bson::{Bson, Document};

use async_trait::async_trait;
use bson::doc;
use futures::StreamExt;
use mongodb::options::FindOptions;

#[async_trait]
pub trait IAnimalRepository {
    async fn add(&self, entity: AnimalEntity) -> String;
    async fn delete(&self, entity: AnimalEntity) -> bool;
    async fn findone(&self, id: String) -> AnimalEntity;
    async fn findmany(&self, filter: Document) -> Vec<AnimalEntity>;
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

    async fn findone(&self, id: String) -> AnimalEntity {
        let filter = doc! {"_id":id};
        let result = self.collection.find_one(filter, None).await;
        match result {
            Ok(r) => {
                let basn = Bson::Document(r.unwrap());
                let b = bson::from_bson(basn);
                b.unwrap()
            }
            Err(e) => {
                print!("{:?}", e);
                AnimalEntity::new()
            }
        }
    }

    async fn findmany(&self, filter: Document) -> Vec<AnimalEntity> {
        let find_options = FindOptions::builder().sort(doc! { "name": 1 }).build();
        let mut cursor = self.collection.find(filter, find_options).await.unwrap();

        let mut animals = Vec::<AnimalEntity>::new();
        while let Some(result) = cursor.next().await {
            let basn = Bson::Document(result.unwrap());
            let b = bson::from_bson(basn);
            let animal = b.unwrap();
            animals.push(animal)
        }
        animals
    }
}
