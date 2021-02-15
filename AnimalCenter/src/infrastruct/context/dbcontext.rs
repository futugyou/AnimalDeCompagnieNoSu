use crate::infrastruct::{config::Config, custom_error::*};

use async_trait::async_trait;
use mongodb::{options::ClientOptions, Client};

#[async_trait]
pub trait IDbContext {
    async fn get_db_context(&self) -> Result<Client, CustomError>;
}

pub struct DBContext {}

#[async_trait]
impl IDbContext for DBContext {
    #[tracing::instrument(skip(self))]
    async fn get_db_context(&self) -> Result<Client, CustomError> {
        let _config = Config::new();
        let conn_str = _config.mongodb_uri;
        let client_options = ClientOptions::parse(conn_str.as_str()).await?;
        Ok(Client::with_options(client_options)?)
    }
}
