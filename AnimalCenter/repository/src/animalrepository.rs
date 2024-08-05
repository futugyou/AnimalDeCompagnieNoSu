use entity::animalentity::AnimalEntity;
use infrastruct::config::Config;
use infrastruct::context::dbcontext::{DBContext, IDbContext};
use model::PageModel;
use tool::custom_error::*;
use tool::stringtoObjectId;

use async_trait::async_trait;
use bson::doc;
use bson::{Bson, Document};
use futures::StreamExt;

#[async_trait]
pub trait IAnimalRepository {
    async fn add(&self, entity: AnimalEntity) -> Result<String, CustomError>;
    async fn delete(&self, entity: AnimalEntity) -> Result<bool, CustomError>;
    async fn findone(&self, id: String) -> Result<AnimalEntity, CustomError>;
    async fn findaggregateone(&self, id: String) -> Result<AnimalEntity, CustomError>;
    async fn findmany(
        &self,
        filter: Document,
        pageing: Option<PageModel>,
    ) -> Result<Vec<AnimalEntity>, CustomError>;
    async fn update(&self, entity: &AnimalEntity) -> Result<bool, CustomError>;
}

#[derive(Debug)]
pub struct AnimalRepository {
    collection: mongodb::Collection<AnimalEntity>,
}

impl AnimalRepository {
    pub async fn new() -> Self {
        let _config = Config::new();
        let table_name = _config.table_name;
        let dbcontext = DBContext {};
        let dbclient = dbcontext.get_db_context().await.unwrap();
        let collection = dbclient
            .database(&table_name)
            .collection(AnimalEntity::get_collection_name());
        Self { collection }
    }
}

#[async_trait]
impl IAnimalRepository for AnimalRepository {
    #[tracing::instrument(skip(self))]
    async fn add(&self, entity: AnimalEntity) -> Result<String, CustomError> {
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
    async fn delete(&self, entity: AnimalEntity) -> Result<bool, CustomError> {
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

    async fn findone(&self, id: String) -> Result<AnimalEntity, CustomError> {
        let oid = stringtoObjectId(&id)?;
        let filter = doc! {"_id":oid};
        let result = self.collection.find_one(filter).await?;
        let mut animal = AnimalEntity::new();
        if let Some(doc) = result {
            animal = doc;
        }
        tracing::info!("findone result: {:#?}", animal);
        Ok(animal)
    }

    #[tracing::instrument(skip(self))]
    async fn findaggregateone(&self, id: String) -> Result<AnimalEntity, CustomError> {
        let oid = stringtoObjectId(&id)?;
        let filter = doc! {"_id":oid};
        let mut subline = Vec::<Bson>::new();
        subline.push(Bson::from("$avatardata"));
        subline.push(Bson::from(0));
        let pipeline = vec![
            doc! {"$match":filter.clone()},
            doc! {"$addFields":doc!{
                "imageid":doc!{"$toObjectId":"$avatar"}
            }},
            doc! {"$lookup":{
                "from":"fileupload",
                "localField":"imageid",
                "foreignField":"_id",
                "as":"avatardata",
            }},
            doc! {"$replaceRoot":doc!{"newRoot":doc!{"$mergeObjects":vec![Bson::from(doc!{"$arrayElemAt":subline}),Bson::from("$$ROOT")]}}},
            doc! {"$project":{
                "imageid":0,
                "avatar":0,
                "ext":0,
                "avatardata":0,
            }},
            doc! {"$project":{
                "name":1,
                "type":1,
                "birthday":1,
                "sub_type":1,
                "idcard":1,
                "rescue_date":1,
                "photoes":1,
                "avatar":"$base64src",
            }},
        ];
        let mut cursor = self.collection.aggregate(pipeline).await?;
        let mut animal = AnimalEntity::new();
        while let Some(result) = cursor.next().await {
            println!("{:?}", result);
            animal = bson::from_bson(Bson::Document(result?))?;
        }
        tracing::info!("findaggregateone result: {:#?}", animal);
        Ok(animal)
    }

    #[tracing::instrument(skip(self))]
    async fn findmany(
        &self,
        filter: Document,
        pageing: Option<PageModel>,
    ) -> Result<Vec<AnimalEntity>, CustomError> {
        let mut skip: u64 = 0;
        let mut limit: i64 = 0;
        if let Some(page) = pageing {
            skip = (page.pageindex * page.pagesize) as u64;
            limit = page.pagesize;
        }

        let mut cursor_find = self.collection.find(filter);
        if skip > 0 {
            cursor_find = cursor_find.skip(skip);
        }
        if limit > 0 {
            cursor_find = cursor_find.limit(limit);
        }
        let mut cursor = cursor_find.sort(doc! {}).await?;
        let mut animals = Vec::<AnimalEntity>::new();
        while let Some(result) = cursor.next().await {
            animals.push(result?);
        }
        tracing::info!("findmany result: {:#?}", animals);
        Ok(animals)
    }

    #[tracing::instrument(skip(self))]
    async fn update(&self, entity: &AnimalEntity) -> Result<bool, CustomError> {
        let oid = stringtoObjectId(&entity.id)?;
        let filter = doc! {"_id":oid};
        let raw: Document = entity.into();
        let update = doc! {"$set" : raw};
        let result = self.collection.update_one(filter, update).await?;
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
