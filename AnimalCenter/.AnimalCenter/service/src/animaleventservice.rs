use model::{
    animalevent::animaleventmodel::{AnimalEventAddRequest, AnimalEventSearchResponse},
    BaseRequest,
};
use repository::animaleventrepository::{AnimalEventRepository, IAnimalEventRepository};
use tool::custom_error::CustomError;

use async_trait::async_trait;

#[async_trait]
pub trait IAnimalEventService {
    async fn addnewevent(&self, request: AnimalEventAddRequest) -> Result<bool, CustomError>;
    async fn getanimaleventlist(&self, animalid: String) -> Vec<AnimalEventSearchResponse>;
}

pub struct AnimalEventService {
    repository: Box<dyn IAnimalEventRepository + Send + Sync>,
}

impl AnimalEventService {
    pub async fn new() -> Self {
        let repository = AnimalEventRepository::new().await;
        Self {
            repository: Box::new(repository),
        }
    }
}

#[async_trait]
impl IAnimalEventService for AnimalEventService {
    #[tracing::instrument(skip(self))]
    async fn addnewevent(&self, request: AnimalEventAddRequest) -> Result<bool, CustomError> {
        request.valid()?;
        let _id = self.repository.add(request.into()).await?;
        Ok(true)
    }

    #[tracing::instrument(skip(self))]
    async fn getanimaleventlist(&self, animalid: String) -> Vec<AnimalEventSearchResponse> {
        let mut response = Vec::<AnimalEventSearchResponse>::new();
        if let Ok(result) = self.repository.find(animalid).await {
            for elem in result {
                response.push(elem.into());
            }
        }
        response
    }
}
