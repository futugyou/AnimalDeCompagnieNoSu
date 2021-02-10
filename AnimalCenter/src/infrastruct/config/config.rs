use config::ConfigError;
use serde::Deserialize;

pub trait IConfig {}

#[derive(Deserialize)]
pub struct Config {
    pub mongodb_uri: String,
    pub api_key: String,
    pub api_value: String,
}

impl Config {
    pub fn new() -> Self {
        Self::from_env().unwrap()
    }

    fn from_env() -> Result<Self, ConfigError> {
        let mut cfg = ::config::Config::new();
        cfg.merge(::config::Environment::new())?;
        cfg.try_into()
    }
}
impl IConfig for Config {}
