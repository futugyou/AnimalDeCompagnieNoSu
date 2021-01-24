use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Animal {
    name: String,
    name_type: String,
}
