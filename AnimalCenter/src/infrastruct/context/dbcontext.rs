use crate::infrastruct::config::Config;
use async_trait::async_trait;
use mongodb::{options::ClientOptions, Client};
#[async_trait]
pub trait IDbContext {
    async fn get_db_context(&self) -> Client;
}

pub struct DBContext {}

#[async_trait]
impl IDbContext for DBContext {
    async fn get_db_context(&self) -> Client {
        let _config = Config::new();
        let conn_str = _config.mongodb_uri;
        let client_options = ClientOptions::parse(&conn_str[..]).await.unwrap();
        Client::with_options(client_options).unwrap()
    }
}
