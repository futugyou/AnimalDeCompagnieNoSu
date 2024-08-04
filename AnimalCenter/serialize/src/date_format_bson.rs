use chrono::{DateTime, Utc};
use serde::{self, Deserialize, Deserializer, Serializer};
use tool::{DATETIMEDEFAULT, FORMAT};

pub fn serialize<S>(date: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match date {
        Some(d) => {
            let s = format!("{}", d.format(FORMAT));
            serializer.serialize_str(&s)
        }
        None => serializer.serialize_str(DATETIMEDEFAULT),
    }
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = std::collections::HashMap::<String, String>::deserialize(deserializer);
    let mut str = String::from(DATETIMEDEFAULT);
    let _ = match s {
        Ok(a) => {
            str = a.get("$date").unwrap().to_string();
        }
        Err(_) => (),
    };

    let r = DateTime::parse_from_str(&str, FORMAT);
    match r {
        Ok(t) => Ok(Option::from(t.to_utc())),
        Err(e) => Err(serde::de::Error::custom(e)),
    }
}
