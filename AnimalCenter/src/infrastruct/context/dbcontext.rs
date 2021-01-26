use crate::infrastruct::config::{Config, IConfig};
use async_trait::async_trait;
use mongodb::{options::ClientOptions, Client};
use std::sync::Mutex;
#[async_trait]
pub trait IDbContext {
    async fn get_db_context(&self) -> Mutex<Client>;
}

pub struct DBContext {}

#[async_trait]
impl IDbContext for DBContext {
    async fn get_db_context(&self) -> Mutex<Client> {
        let _config = Config {};
        let conn_str = _config.get_config_with_key("MONGODB_URI");
        let client_options = ClientOptions::parse(&conn_str[..]).await.unwrap();
        Mutex::new(Client::with_options(client_options).unwrap())
    }
}
