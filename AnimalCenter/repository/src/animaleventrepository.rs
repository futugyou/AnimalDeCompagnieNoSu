use async_trait::async_trait;
use bson::doc;
use bson::{Bson, Document};
use entity::animaeventlentity::AnimalEventEntity;
use futures::StreamExt;
use infrastruct::context::dbcontext::{DBContext, IDbContext};
use mongodb::options::FindOptions;
use tool::custom_error::*;
use tool::stringtoObjectId;

#[async_trait]

pub trait IAnimalEventRepository {
    async fn add(&self, entity: AnimalEventEntity) -> Result<String, CustomError>;
    async fn delete(&self, id: String) -> Result<bool, CustomError>;
    async fn find(&self, animalid: String) -> Result<Vec<AnimalEventEntity>, CustomError>;
}

#[derive(Debug)]
pub struct AnimalEventRepository {
    collection: mongodb::Collection,
}

impl AnimalEventRepository {
    pub async fn new() -> Self {
        let dbcontext = DBContext {};
        let dbclient = dbcontext.get_db_context().await.unwrap();
        let collection = dbclient
            .database("react-app")
            .collection(AnimalEventEntity::get_collection_name());
        Self { collection }
    }
}

#[async_trait]
impl IAnimalEventRepository for AnimalEventRepository {
    #[tracing::instrument(skip(self))]
    async fn add(&self, entity: AnimalEventEntity) -> Result<String, CustomError> {
        let docs: Document = entity.into();
        let result = self.collection.insert_one(docs, None).await?;
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
    async fn delete(&self, id: String) -> Result<bool, CustomError> {
        let oid = stringtoObjectId(&id)?;
        let filter = doc! {"_id":oid};
        let result = self.collection.delete_one(filter, None).await?;
        if result.deleted_count == 0 {
            tracing::warn!("db delete_one result: id {:#?} can not found in db", &id);
            return Ok(false);
        }
        Ok(true)
    }

    #[tracing::instrument(skip(self))]
    async fn find(&self, animalid: String) -> Result<Vec<AnimalEventEntity>, CustomError> {
        let find_options = FindOptions::builder().sort(doc! {}).build();
        let filter = doc! {"animalid":animalid};
        let mut cursor = self.collection.find(filter, find_options).await?;
        let mut animals = Vec::<AnimalEventEntity>::new();
        while let Some(result) = cursor.next().await {
            animals.push(bson::from_bson(Bson::Document(result?))?);
        }
        tracing::info!("findmany result: {:#?}", animals);
        Ok(animals)
    }
}
