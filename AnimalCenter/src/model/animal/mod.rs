use validator::Validate;

use crate::infrastruct::custom_error::CustomError;

pub mod animaldeletemodel;
pub mod animalinsertmodel;
pub mod animalsearchmodel;
pub mod animalupdatemodel;

pub struct AnimalClearFakeData {}

pub trait BaseRequest: Validate {
    fn valid(&self) -> Result<bool, CustomError> {
        self.validate()?;
        Ok(true)
    }
}
