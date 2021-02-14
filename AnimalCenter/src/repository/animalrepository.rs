use crate::{
    entity::animalentity::AnimalEntity,
    infrastruct::{
        context::dbcontext::{DBContext, IDbContext},
        custom_error::{CustomError, CustomErrorKind},
        stringtoObjectId,
    },
};

use async_trait::async_trait;
use bson::doc;
use bson::{Bson, Document};
use futures::StreamExt;
use mongodb::options::FindOptions;

#[async_trait]
pub trait IAnimalRepository {
    async fn add(&self, entity: AnimalEntity) -> Result<String, CustomError>;
    async fn delete(&self, entity: AnimalEntity) -> Result<bool, CustomError>;
    async fn findone(&self, id: String) -> Result<AnimalEntity, CustomError>;
    async fn findmany(&self, filter: Document) -> Vec<AnimalEntity>;
    async fn update(&self, entity: AnimalEntity) -> Result<bool, String>;
}

#[derive(Debug)]
pub struct AnimalRepository {
    collection: mongodb::Collection,
}

impl AnimalRepository {
    pub async fn new() -> Self {
        let dbcontext = DBContext {};
        let dbclient = dbcontext.get_db_context().await.unwrap();
        let collection = dbclient
            .database("react-app")
            .collection(AnimalEntity::get_collection_name());
        Self { collection }
    }
}

#[async_trait]
impl IAnimalRepository for AnimalRepository {
    #[tracing::instrument(skip(self))]
    async fn add(&self, entity: AnimalEntity) -> Result<String, CustomError> {
        let docs = doc! {
                "name": entity.name,
                "type": entity.animal_type,
                "birthday":entity.birthday.unwrap(),
                "sub_type":entity.sub_type,
                "idcard":entity.idcard,
        };
        let result = self.collection.insert_one(docs, None).await?;
        tracing::info!("db insert_one result: {:#?}", result);
        if let Bson::ObjectId(oid) = result.inserted_id {
            return Ok(oid.to_hex());
        }
        Err(CustomError::new(
            "40000".to_owned(),
            "insert_one return an unknown data type".to_owned(),
            CustomErrorKind::MiddlewareError,
        ))
    }

    #[tracing::instrument(skip(self))]
    async fn update(&self, entity: AnimalEntity) -> Result<bool, String> {
        let filter = doc! {"_id":bson::oid::ObjectId::with_string(&entity.id).unwrap() };
        let update = doc! {"$set" : doc!{
                "name": entity.name,
                "type": entity.animal_type,
                "birthday":entity.birthday.unwrap(),
                "sub_type":entity.sub_type,
        }};

        let result = self.collection.update_one(filter, update, None).await;
        match result {
            Ok(r) => {
                tracing::info!("db update_one result: {:#?}", r);
                Ok(true)
            }
            Err(e) => {
                tracing::error!("db update_one error: {:#?}", e);
                Err(e.to_string())
            }
        }
    }

    #[tracing::instrument(skip(self))]
    async fn delete(&self, entity: AnimalEntity) -> Result<bool, CustomError> {
        let oid = stringtoObjectId(&entity.id)?;
        let filter = doc! {"_id":oid};
        let result = self.collection.delete_one(filter, None).await?;
        if result.deleted_count == 0 {
            tracing::warn!(
                "db delete_one result: id {:#?} can not found in db",
                &entity.id
            );
            return Ok(false);
        }
        Ok(true)
    }

    #[tracing::instrument(skip(self))]
    async fn findone(&self, id: String) -> Result<AnimalEntity, CustomError> {
        let oid = stringtoObjectId(&id)?;
        let filter = doc! {"_id":oid};
        let result = self.collection.find_one(filter, None).await?;
        let mut animal = AnimalEntity::new();

        if let Some(doc) = result {
            animal = bson::from_bson(Bson::Document(doc))?;
        }
        tracing::info!("findone result: {:#?}", animal);
        Ok(animal)
    }

    #[tracing::instrument(skip(self))]
    async fn findmany(&self, filter: Document) -> Vec<AnimalEntity> {
        let find_options = FindOptions::builder().sort(doc! { "name": 1 }).build();
        let cursor_result = self.collection.find(filter, find_options).await;
        let mut animals = Vec::<AnimalEntity>::new();
        match cursor_result {
            Ok(mut cursor) => {
                while let Some(result) = cursor.next().await {
                    let basn = Bson::Document(result.unwrap());
                    let b = bson::from_bson(basn);
                    match b {
                        Ok(animal) => animals.push(animal),
                        Err(e) => {
                            tracing::error!("bson to animal_entity error when findmany: {:#?}", e);
                        }
                    }
                }
            }
            Err(e) => {
                tracing::error!("DB error when findmany: {:#?}", e);
            }
        }
        tracing::info!("findmany result: {:#?}", animals);
        animals
    }
}
