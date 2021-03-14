use serde::*;

#[derive(Debug, Default, Deserialize)]
pub struct RescueClassificationRequest {
    #[serde(default = "default_rescueh")]
    rescueh_classification: String,
}

#[derive(Debug, Default, Serialize)]
pub struct RescueClassificationResponse {
    #[serde(default)]
    classification: String,
    #[serde(default)]
    count: u32,
}

fn default_rescueh() -> String {
    "age".to_owned()
}
