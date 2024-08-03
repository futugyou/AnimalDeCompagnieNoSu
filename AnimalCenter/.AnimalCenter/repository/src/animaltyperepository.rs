use entity::animaltypeentity::AnimalTypeEntity;
use infrastruct::context::dbcontext::{DBContext, IDbContext};
use tool::custom_error::*;
use tool::stringtoObjectId;

use async_trait::async_trait;
use bson::doc;
use bson::{Bson, Document};
use futures::StreamExt;
use mongodb::options::FindOptions;

#[async_trait]
pub trait IAnimalTypeRepository {
    async fn add(&self, entity: AnimalTypeEntity) -> Result<String, CustomError>;
    async fn delete(&self, entity: AnimalTypeEntity) -> Result<bool, CustomError>;
    async fn findone(&self, id: String) -> Result<AnimalTypeEntity, CustomError>;
    async fn findmany(&self, filter: Document) -> Result<Vec<AnimalTypeEntity>, CustomError>;
    async fn update(&self, entity: AnimalTypeEntity) -> Result<bool, CustomError>;
}

#[derive(Debug)]
pub struct AnimalTypeRepository {
    collection: mongodb::Collection::<AnimalTypeEntity>,
}

impl AnimalTypeRepository {
    pub async fn new() -> Self {
        let dbcontext = DBContext {};
        let dbclient = dbcontext.get_db_context().await.unwrap();
        let collection = dbclient
            .database("react-app")
            .collection(AnimalTypeEntity::get_collection_name());
        Self { collection }
    }
}

#[async_trait]
impl IAnimalTypeRepository for AnimalTypeRepository {
    #[tracing::instrument(skip(self))]
    async fn add(&self, entity: AnimalTypeEntity) -> Result<String, CustomError> {
        let result = self.collection.insert_one(entity, None).await?;
        tracing::info!("db insert_one result: {:#?}", result);
        if let Bson::ObjectId(oid) = result.inserted_id {
            return Ok(oid.to_hex());
        }
        Err(CustomError::new(
            "40000".to_owned(),
            "insert_one return an unknown data type".to_owned(),
            CustomErrorKind::MongodbError,
        ))
    }
    #[tracing::instrument(skip(self))]
    async fn delete(&self, entity: AnimalTypeEntity) -> Result<bool, CustomError> {
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
    async fn findone(&self, id: String) -> Result<AnimalTypeEntity, CustomError> {
        let oid = stringtoObjectId(&id)?;
        let filter = doc! {"_id":oid};
        let result = self.collection.find_one(filter, None).await?;
        let mut animaltype = AnimalTypeEntity::new();

        if let Some(doc) = result {
            animaltype = doc;
        }
        tracing::info!("findone result: {:#?}", animaltype);
        Ok(animaltype)
    }
    #[tracing::instrument(skip(self))]
    async fn findmany(&self, filter: Document) -> Result<Vec<AnimalTypeEntity>, CustomError> {
        let find_options = FindOptions::builder()
            .sort(doc! { "animal_type": 1 })
            .build();
        let mut cursor = self.collection.find(filter, find_options).await?;
        let mut animaltypes = Vec::<AnimalTypeEntity>::new();
        while let Some(result) = cursor.next().await {
            animaltypes.push(result?);
        }
        tracing::info!("findmany result: {:#?}", animaltypes);
        Ok(animaltypes)
    }
    #[tracing::instrument(skip(self))]
    async fn update(&self, entity: AnimalTypeEntity) -> Result<bool, CustomError> {
        let oid = stringtoObjectId(&entity.id)?;
        let filter = doc! {"_id":oid};
        let update = doc! {"$set" : doc!{
                "pid": entity.pid,
                "type": entity.animal_type,
        }};

        let result = self.collection.update_one(filter, update, None).await?;
        if result.matched_count == 0 && result.modified_count == 0 {
            tracing::warn!(
                "db update_one result: id {:#?} can not found in db",
                &entity.id
            );
            return Ok(false);
        }
        Ok(true)
    }
}
