use crate::infrastruct::serialize::from_str;
use serde::{Deserialize, Serialize};

pub mod animal;
pub mod animaltype;
pub mod file;
pub mod report;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PageModel {
    #[serde(deserialize_with = "from_str")]
    pub pageindex: u64,
    #[serde(deserialize_with = "from_str")]
    pub pagesize: u64,
}
