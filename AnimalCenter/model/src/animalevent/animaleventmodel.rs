use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serialize::*;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct AnimalEventSearchResponse {
    #[serde(default)]
    pub event: String,
    #[serde(with = "date_format", default)]
    pub event_time: Option<DateTime<Utc>>,
}
