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
        let builder = ::config::Config::builder().add_source(::config::Environment::default());
        let app: Config = builder.build().unwrap().try_deserialize().unwrap();
        Ok(app)
    }
}
impl IConfig for Config {}
