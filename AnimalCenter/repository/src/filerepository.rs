use entity::fileentity::FileEntity;
use infrastruct::config::Config;
use infrastruct::context::dbcontext::{DBContext, IDbContext};
use tool::custom_error::*;
use tool::stringtoObjectId;

use async_trait::async_trait;
use bson::doc;
use bson::Bson;
use futures::StreamExt;

#[async_trait]
pub trait IFileRepository {
    async fn add(&self, entity: FileEntity) -> Result<String, CustomError>;
    async fn addmany(&self, entitys: Vec<FileEntity>) -> Result<Vec<String>, CustomError>;
    async fn delete(&self, entity: FileEntity) -> Result<bool, CustomError>;
    async fn findone(&self, id: String) -> Result<FileEntity, CustomError>;
    async fn findmany(&self, ids: Vec<String>) -> Result<Vec<FileEntity>, CustomError>;
}

#[derive(Debug)]
pub struct FileRepository {
    collection: mongodb::Collection<FileEntity>,
}

impl FileRepository {
    pub async fn new() -> Self {
        let _config = Config::new();
        let table_name = _config.table_name;
        let dbcontext = DBContext {};
        let dbclient = dbcontext.get_db_context().await.unwrap();
        let collection = dbclient
            .database(&table_name)
            .collection(FileEntity::get_collection_name());
        Self { collection }
    }
}

#[async_trait]
impl IFileRepository for FileRepository {
    #[tracing::instrument(skip(self))]
    async fn add(&self, entity: FileEntity) -> Result<String, CustomError> {
        let result = self.collection.insert_one(entity).await?;
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
        let result = self.collection.delete_one(filter).await?;
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
        let result = self.collection.find_one(filter).await?;
        let mut entity = FileEntity::default();

        if let Some(doc) = result {
            entity = doc;
        }
        tracing::info!("findone result: {:#?}", entity);
        Ok(entity)
    }

    #[tracing::instrument(skip(self))]
    async fn findmany(&self, ids: Vec<String>) -> Result<Vec<FileEntity>, CustomError> {
        let mut filter = doc! {};
        if ids.len() > 0 {
            filter = doc! {
                "_id": doc!{
                    "$in": ids.iter().map(|id| stringtoObjectId(&id).unwrap() ).collect::<Vec<bson::oid::ObjectId>>()}
            };
        }

        let mut cursor = self
            .collection
            .find(filter)
            .sort(doc! { "name": 1 })
            .await?;
        let mut animals = Vec::<FileEntity>::new();
        while let Some(result) = cursor.next().await {
            animals.push(result?);
        }
        tracing::info!("findmany result: {:#?}", animals);
        Ok(animals)
    }

    async fn addmany(&self, entitys: Vec<FileEntity>) -> Result<Vec<String>, CustomError> {
        let result = self.collection.insert_many(entitys).await?;
        tracing::info!("db insert_many result: {:#?}", result);
        if result.inserted_ids.len() > 0 {
            let ids = result
                .inserted_ids
                .into_iter()
                // .map(|(_x, y)| {
                //     let mut id = "".to_owned();
                //     if let Bson::ObjectId(oid) = y {
                //         id = oid.to_hex();
                //     }
                //     id
                // })
                .flat_map(|(_, y)| {
                    if let Bson::ObjectId(oid) = y {
                        Some(oid.to_hex())
                    } else {
                        None
                    }
                })
                .collect::<Vec<String>>();
            Ok(ids)
        } else {
            Err(CustomError::new(
                "40000".to_owned(),
                "insert_one return an unknown data type".to_owned(),
                CustomErrorKind::MongodbError,
            ))
        }
    }
}
