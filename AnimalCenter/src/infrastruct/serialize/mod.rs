pub mod date_format;
pub mod date_format_bson;
#[allow(dead_code)]
pub mod naive_date_format;
#[allow(dead_code)]
pub mod naive_date_format_bson;

use serde::{de, Deserialize, Deserializer};
use std::fmt::Display;
use std::str::FromStr;

pub fn deserialize_object_id<'de, S, D>(deserializer: D) -> Result<S, D::Error>
where
    S: FromStr,
    S::Err: Display,
    D: Deserializer<'de>,
{
    let ss = std::collections::HashMap::<String, String>::deserialize(deserializer);
    let mut str = String::from("");
    let _ = match ss {
        Ok(a) => {
            str = a.get("$oid").unwrap().to_string();
        }
        Err(_) => (),
    };
    S::from_str(&str).map_err(de::Error::custom)
}
