use async_trait::async_trait;
use model::report::rescue_classification::*;
use repository::reportrepository::*;

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
        let mut result: Vec<RescueClassificationResponse> = Vec::new();
        let exec_result = self.repository.get_age_rescue_data(request.into()).await;
        match exec_result {
            Ok(res) => result = res.into_iter().map(|x| x.into()).collect(),
            Err(error) => tracing::error!("call  get_age_rescue_data error: {:#?}", error),
        }
        result
    }
}
