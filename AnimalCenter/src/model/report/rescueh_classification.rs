use serde::*;

#[derive(Debug, Default, Deserialize)]
pub struct RescuehClassificationRequest {
    #[serde(default = "age")]
    rescueh_classification: String,
}

#[derive(Debug, Default, Serialize)]
pub struct RescuehClassificationResponse {
    #[serde(default)]
    classification: String,
    #[serde(default)]
    count: u32,
}
