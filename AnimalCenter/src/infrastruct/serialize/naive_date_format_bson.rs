use crate::infrastruct::{NAIVE_DATETIMEDEFAULT, NAIVE_FORMAT};

use chrono::NaiveDateTime;
use serde::{self, Deserialize, Deserializer, Serializer};

pub fn serialize<S>(date: &Option<NaiveDateTime>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match date {
        Some(d) => {
            let s = format!("{}", d.format(NAIVE_FORMAT));
            serializer.serialize_str(&s)
        }
        None => serializer.serialize_str(NAIVE_DATETIMEDEFAULT),
    }
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveDateTime>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = std::collections::HashMap::<String, String>::deserialize(deserializer);
    let mut str = String::from(NAIVE_DATETIMEDEFAULT);
    let _ = match s {
        Ok(a) => {
            str = a.get("$date").unwrap().to_string();
        }
        Err(_) => (),
    };

    let r = NaiveDateTime::parse_from_str(&str, NAIVE_FORMAT);
    match r {
        Ok(t) => Ok(Option::from(t)),
        Err(e) => Err(serde::de::Error::custom(e)),
    }
}
