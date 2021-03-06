use serde::Deserialize;
use tool::custom_error::CustomError;

pub trait IConfig {}

#[derive(Deserialize)]
pub struct Config {
    pub mongodb_uri: String,
    pub api_key: String,
    pub api_value: String,
    pub amqp_addr: String,
    pub server_address: String,
    pub file_upload_path: String,
}

impl Config {
    pub fn new() -> Self {
        Self::from_env().unwrap()
    }

    fn from_env() -> Result<Self, CustomError> {
        let mut cfg = ::config::Config::new();
        cfg.merge(::config::Environment::new())?;
        Ok(cfg.try_into()?)
    }
}
impl IConfig for Config {}
