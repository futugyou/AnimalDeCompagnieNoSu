use crate::{entity::rescueentity::RescueEntity, infrastruct::context::dbcontext::*};
use async_trait::async_trait;
use bson::Document;

#[async_trait]
pub trait IReportRepository {
    async fn get_age_rescue_data(&self, req: Document) -> Vec<RescueEntity>;
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
    async fn get_age_rescue_data(&self, req: Document) -> Vec<RescueEntity> {
        todo!()
    }
}
