use async_trait::async_trait;

use crate::{model::report::rescueh_classification::*, repository::reportrepository::*};

#[async_trait]
pub trait IReportService {
    async fn get_rescue_classification(
        &self,
        request: RescuehClassificationRequest,
    ) -> Vec<RescuehClassificationResponse>;
}

pub struct ReportService {
    repository: Box<dyn IReportRepository + Send + Sync>,
}

impl ReportService {
    pub async fn new() -> Self {
        let repository = ReportRepository::new().await;
        Self {
            repository: Box::new(repository),
        }
    }
}

#[async_trait]
impl IReportService for ReportService {
    #[tracing::instrument(skip(self))]
    async fn get_rescue_classification(
        &self,
        request: RescuehClassificationRequest,
    ) -> Vec<RescuehClassificationResponse> {
        vec![RescuehClassificationResponse::default()]
    }
}
