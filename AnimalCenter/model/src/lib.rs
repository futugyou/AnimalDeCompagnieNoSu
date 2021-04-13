#[macro_use]
extern crate prost_derive;

use serde::{Deserialize, Serialize};
use serialize::from_str;
use tool::custom_error::CustomError;
use validator::Validate;

pub mod animal;
pub mod animalevent;
pub mod animaltype;
pub mod file;
pub mod report;
pub mod structmapper;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageModel {
    #[serde(deserialize_with = "from_str")]
    pub pageindex: i64,
    #[serde(deserialize_with = "from_str")]
    pub pagesize: i64,
}

pub trait BaseRequest: Validate {
    fn valid(&self) -> Result<bool, CustomError> {
        self.validate()?;
        Ok(true)
    }
}
