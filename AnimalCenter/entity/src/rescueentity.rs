use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RescueEntity {
    pub count: u32,
    pub classification: String,
}
