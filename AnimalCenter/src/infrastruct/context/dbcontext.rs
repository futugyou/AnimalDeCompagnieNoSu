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
        let client_options_result = ClientOptions::parse(conn_str.as_str()).await;
        match client_options_result {
            Ok(client_options) => Ok(Client::with_options(client_options).unwrap()),
            Err(e) => {
                let custom_error = CustomError::new(
                    "40000".to_owned(),
                    e.to_string(),
                    CustomErrorKind::MiddlewareError,
                );
                tracing::error!("mongodb client get error: {:#?}", custom_error);
                Err(custom_error)
            }
        }
    }
}
