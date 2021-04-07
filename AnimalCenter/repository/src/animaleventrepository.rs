use async_trait::async_trait;
use entity::animaeventlentity::AnimalEventEntity;
use infrastruct::context::dbcontext::{DBContext, IDbContext};
use tool::custom_error::CustomError;

#[async_trait]

pub trait IAnimalEventRepository {
    async fn add(&self, entity: AnimalEventEntity) -> Result<String, CustomError>;
    async fn delete(&self, id: String) -> Result<bool, CustomError>;
    async fn find(&self, animalid: String) -> Vec<AnimalEventEntity>;
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
        todo!()
    }

    #[tracing::instrument(skip(self))]
    async fn delete(&self, id: String) -> Result<bool, CustomError> {
        todo!()
    }

    #[tracing::instrument(skip(self))]
    async fn find(&self, animalid: String) -> Vec<AnimalEventEntity> {
        todo!()
    }
}
