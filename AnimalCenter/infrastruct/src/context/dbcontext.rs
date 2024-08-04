use crate::config::Config;
use async_once::AsyncOnce;
use async_trait::async_trait;
use lazy_static::lazy_static;
use mongodb::{options::ClientOptions, Client};
use once_cell::sync::Lazy;
use std::sync::Arc;
use tokio::sync::Mutex;
use tool::custom_error::CustomError;

lazy_static! {
    static ref LAZY_CONNECTION: AsyncOnce<Result<Client, CustomError>> = AsyncOnce::new(async {
        let _config = Config::new();
        let conn_str = _config.mongodb_uri;
        let client_options = ClientOptions::parse(conn_str.as_str()).await?;
        Ok(Client::with_options(client_options)?)
    });
}

static LAZY_CONNECTION1: Lazy<Mutex<Option<Arc<Client>>>> = Lazy::new(|| Mutex::new(None));

async fn initialize_client() -> Result<Arc<Client>, CustomError> {
    let _config = Config::new();
    let conn_str = _config.mongodb_uri;
    let client_options = ClientOptions::parse(conn_str.as_str()).await?;
    let client = Client::with_options(client_options)?;
    Ok(Arc::new(client))
}

async fn get_client() -> Result<Arc<Client>, CustomError> {
    let mut lock = LAZY_CONNECTION1.lock().await;

    if lock.is_none() {
        match initialize_client().await {
            Ok(client) => {
                *lock = Some(client.clone());
                Ok(client)
            }
            Err(e) => Err(e),
        }
    } else {
        Ok(lock.as_ref().unwrap().clone())
    }
}

#[async_trait]
pub trait IDbContext {
    async fn get_db_context(&self) -> Result<Client, CustomError>;
    async fn get_db_context_arc(&self) -> Result<Arc<Client>, CustomError>;
}

pub struct DBContext {}

#[async_trait]
impl IDbContext for DBContext {
    #[tracing::instrument(skip(self))]
    async fn get_db_context(&self) -> Result<Client, CustomError> {
        LAZY_CONNECTION.get().await.to_owned()
    }
    #[tracing::instrument(skip(self))]
    async fn get_db_context_arc(&self) -> Result<Arc<Client>, CustomError> {
        get_client().await
    }
}
