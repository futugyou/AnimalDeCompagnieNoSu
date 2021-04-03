use crate::config::Config;
use async_once::AsyncOnce;
use async_trait::async_trait;
use lazy_static::lazy_static;
use mongodb::{options::ClientOptions, Client};
use tool::custom_error::CustomError;

lazy_static! {
    static ref LAZY_CONNECTION: AsyncOnce<Result<Client, CustomError>> = AsyncOnce::new(async {
        let _config = Config::new();
        let conn_str = _config.mongodb_uri;
        let client_options = ClientOptions::parse(conn_str.as_str()).await?;
        Ok(Client::with_options(client_options)?)
    });
}

#[async_trait]
pub trait IDbContext {
    async fn get_db_context(&self) -> Result<Client, CustomError>;
}

pub struct DBContext {}

#[async_trait]
impl IDbContext for DBContext {
    #[tracing::instrument(skip(self))]
    async fn get_db_context(&self) -> Result<Client, CustomError> {
        LAZY_CONNECTION.get().await.to_owned()
        // let _config = Config::new();
        // let conn_str = _config.mongodb_uri;
        // let client_options = ClientOptions::parse(conn_str.as_str()).await?;
        // Ok(Client::with_options(client_options)?)
    }
}
