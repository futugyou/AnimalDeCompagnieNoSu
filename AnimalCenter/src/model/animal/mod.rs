use validator::Validate;

use crate::infrastruct::custom_error::CustomError;

pub mod animalmodel;
pub mod animaltypemodel;

pub trait BaseRequest: Validate {
    fn valid(&self) -> Result<bool, CustomError> {
        self.validate()?;
        Ok(true)
    }
}
