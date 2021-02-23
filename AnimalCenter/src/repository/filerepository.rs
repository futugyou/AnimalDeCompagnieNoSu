use crate::{
    entity::fileentity::FileEntity,
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
pub trait IFileRepository {
    async fn add(&self, entity: FileEntity) -> Result<String, CustomError>;
    async fn delete(&self, entity: FileEntity) -> Result<bool, CustomError>;
    async fn findone(&self, id: String) -> Result<FileEntity, CustomError>;
    async fn findmany(&self, ids: Vec<String>) -> Result<Vec<FileEntity>, CustomError>;
}

#[derive(Debug)]
pub struct FileRepository {
    collection: mongodb::Collection,
}

impl FileRepository {
    pub async fn new() -> Self {
        let dbcontext = DBContext {};
        let dbclient = dbcontext.get_db_context().await.unwrap();
        let collection = dbclient
            .database("react-app")
            .collection(FileEntity::get_collection_name());
        Self { collection }
    }
}

#[async_trait]
impl IFileRepository for FileRepository {
    #[tracing::instrument(skip(self))]
    async fn add(&self, entity: FileEntity) -> Result<String, CustomError> {
        let docs = doc! {
                "name": entity.name,
                "ext": entity.ext,
                "base64src":entity.base64src,
        };
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
    async fn delete(&self, entity: FileEntity) -> Result<bool, CustomError> {
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
    async fn findone(&self, id: String) -> Result<FileEntity, CustomError> {
        let oid = stringtoObjectId(&id)?;
        let filter = doc! {"_id":oid};
        let result = self.collection.find_one(filter, None).await?;
        let mut entity = FileEntity::default();

        if let Some(doc) = result {
            entity = bson::from_bson(Bson::Document(doc))?;
        }
        tracing::info!("findone result: {:#?}", entity);
        Ok(entity)
    }

    #[tracing::instrument(skip(self))]
    async fn findmany(&self, ids: Vec<String>) -> Result<Vec<FileEntity>, CustomError> {
        let filter = doc! {
            "_id": doc!{
                "$in": ids.iter().map(|id| stringtoObjectId(&id).unwrap() ).collect::<Vec<bson::oid::ObjectId>>()}
        };
        let find_options = FindOptions::builder().sort(doc! { "name": 1 }).build();
        let mut cursor = self.collection.find(filter, find_options).await?;
        let mut animals = Vec::<FileEntity>::new();
        while let Some(result) = cursor.next().await {
            animals.push(bson::from_bson(Bson::Document(result?))?);
        }
        tracing::info!("findmany result: {:#?}", animals);
        Ok(animals)
    }
}
