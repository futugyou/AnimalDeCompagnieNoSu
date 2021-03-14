use async_trait::async_trait;

use crate::{model::report::rescue_classification::*, repository::reportrepository::*};

#[async_trait]
pub trait IReportService {
    async fn get_rescue_classification(
        &self,
        request: RescueClassificationRequest,
    ) -> Vec<RescueClassificationResponse>;
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
        request: RescueClassificationRequest,
    ) -> Vec<RescueClassificationResponse> {
        //         [
        //     {
        //         '$group': {
        //             '_id': '$sub_type',
        //             'count': {
        //                 '$sum': 1
        //             }
        //         }
        //     }, {
        //         '$project': {
        //             'classification': '$_id',
        //             'count': 1,
        //             '_id': 0
        //         }
        //     }
        // ]
        vec![RescueClassificationResponse::default()]
    }
}
