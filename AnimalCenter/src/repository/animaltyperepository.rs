use crate::{
    entity::animaltypeentity::AnimalTypeEntity,
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
pub trait IAnimalTypeRepository {
    async fn add(&self, entity: AnimalTypeEntity) -> Result<String, CustomError>;
    async fn delete(&self, entity: AnimalTypeEntity) -> Result<bool, CustomError>;
    async fn findone(&self, id: String) -> Result<AnimalTypeEntity, CustomError>;
    async fn findmany(&self, filter: Document) -> Result<Vec<AnimalTypeEntity>, CustomError>;
    async fn update(&self, entity: AnimalTypeEntity) -> Result<bool, CustomError>;
}

#[derive(Debug)]
pub struct AnimalTypeRepository {
    collection: mongodb::Collection,
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
    async fn add(&self, entity: AnimalTypeEntity) -> Result<String, CustomError> {
        todo!()
    }

    async fn delete(&self, entity: AnimalTypeEntity) -> Result<bool, CustomError> {
        todo!()
    }

    async fn findone(&self, id: String) -> Result<AnimalTypeEntity, CustomError> {
        todo!()
    }

    async fn findmany(&self, filter: Document) -> Result<Vec<AnimalTypeEntity>, CustomError> {
        todo!()
    }

    async fn update(&self, entity: AnimalTypeEntity) -> Result<bool, CustomError> {
        todo!()
    }
}
