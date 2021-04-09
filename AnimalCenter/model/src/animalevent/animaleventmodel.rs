use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serialize::*;
use validator::Validate;

use crate::BaseRequest;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct AnimalEventSearchResponse {
    #[serde(default)]
    pub event: String,
    #[serde(with = "date_format", default)]
    pub event_time: Option<DateTime<Utc>>,
}

impl BaseRequest for AnimalEventAddRequest {}
#[derive(Default, Validate, Debug, Serialize, Deserialize)]
pub struct AnimalEventAddRequest {
    #[serde(default)]
    pub animalid: String,
    #[serde(default)]
    pub event_type: String,
    #[serde(default)]
    pub event: String,
    #[serde(with = "date_format", default)]
    pub event_time: Option<DateTime<Utc>>,
}
