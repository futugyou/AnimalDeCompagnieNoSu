use crate::model::animal::filemodel::FileSearchResponse;
use crate::repository::filerepository::FileRepository;
use crate::repository::filerepository::IFileRepository;
use crate::{entity::fileentity::FileEntity, infrastruct::custom_error::*};

use async_trait::async_trait;
#[async_trait]
pub trait IFileService {
    async fn find_file_by_ids(&self, ids: Vec<String>) -> Vec<FileSearchResponse>;
    async fn delete_file(&self, id: String) -> Result<(), CustomError>;
    async fn find_file_by_id(&self, id: String) -> FileSearchResponse;
}

pub struct FileService {
    file_repository: Box<dyn IFileRepository + Send + Sync>,
}

impl FileService {
    pub async fn new() -> Self {
        let file_repository = FileRepository::new().await;
        Self {
            file_repository: Box::new(file_repository),
        }
    }
}

#[async_trait]
impl IFileService for FileService {
    #[tracing::instrument(skip(self))]
    async fn find_file_by_ids(&self, ids: Vec<String>) -> Vec<FileSearchResponse> {
        let serach_result = self.file_repository.findmany(ids).await;
        match serach_result {
            Ok(search) => {
                let mut results = Vec::<FileSearchResponse>::new();
                for elem in search {
                    let response = elem.into();
                    results.push(response);
                }
                tracing::info!("search_animals result: {:#?}", results);
                results
            }
            Err(error) => {
                tracing::error!(
                    "find_file_by_ids call file_repository.findmany() error: {:#?}",
                    error
                );
                Vec::<FileSearchResponse>::new()
            }
        }
    }

    #[tracing::instrument(skip(self))]
    async fn delete_file(&self, id: String) -> Result<(), CustomError> {
        let mut entity = FileEntity::default();
        entity.id = id;
        let deleteresult = self.file_repository.delete(entity).await?;
        if deleteresult {}
        Ok(())
    }

    #[tracing::instrument(skip(self))]
    async fn find_file_by_id(&self, id: String) -> FileSearchResponse {
        let findresult = self.file_repository.findone(id).await;
        match findresult {
            Ok(animal) => animal.into(),
            Err(err) => {
                tracing::warn!("can not found the data , {:#?}", err);
                FileEntity::default().into()
            }
        }
    }
}
