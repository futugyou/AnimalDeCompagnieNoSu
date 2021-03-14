use crate::{entity::animalentity::AnimalEntity, infrastruct::custom_error::CustomError};
use crate::{entity::rescueentity::RescueEntity, infrastruct::context::dbcontext::*};
use async_trait::async_trait;
use bson::{Bson, Document};
use futures::StreamExt;

#[async_trait]
pub trait IReportRepository {
    async fn get_age_rescue_data(
        &self,
        req: Vec<Document>,
    ) -> Result<Vec<RescueEntity>, CustomError>;
}

pub struct ReportRepository {
    context: mongodb::Database,
}

impl ReportRepository {
    pub async fn new() -> Self {
        let dbcontext = DBContext {};
        let dbclient = dbcontext.get_db_context().await.unwrap();
        let context = dbclient.database("react-app");
        Self { context }
    }
}

#[async_trait]
impl IReportRepository for ReportRepository {
    #[tracing::instrument(skip(self))]
    async fn get_age_rescue_data(
        &self,
        pipeline: Vec<Document>,
    ) -> Result<Vec<RescueEntity>, CustomError> {
        let collection = self.context.collection(AnimalEntity::get_collection_name());
        let mut cursor = collection.aggregate(pipeline, None).await?;
        let mut datas = Vec::<RescueEntity>::new();
        while let Some(result) = cursor.next().await {
            let data = bson::from_bson(Bson::Document(result?))?;
            datas.push(data);
        }
        tracing::info!("findaggregateone result: {:#?}", datas);
        Ok(datas)
    }
}
