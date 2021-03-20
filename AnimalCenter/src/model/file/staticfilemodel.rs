use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct StaticfileResponse {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub status: String,
    #[serde(default, rename = "thumbUrl")]
    pub thumb_url: String,
    #[serde(default)]
    pub url: String,
}
