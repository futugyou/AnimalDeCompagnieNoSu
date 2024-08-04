use serde::*;

#[derive(Debug, Default, Deserialize)]
pub struct RescueClassificationRequest {
    #[serde(default = "default_rescue")]
    pub rescue_classification: String,
}

#[derive(Debug, Default, Serialize)]
pub struct RescueClassificationResponse {
    #[serde(default)]
    pub classification: String,
    #[serde(default)]
    pub count: u32,
}

fn default_rescue() -> String {
    "age".to_owned()
}
