use tool::{DATETIMEDEFAULT, FORMAT};

use chrono::{DateTime, TimeZone, Utc};
use serde::{self, Deserialize, Deserializer, Serializer};

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
    let deresult = String::deserialize(deserializer);
    let mut str = String::from(DATETIMEDEFAULT);
    let _ = match deresult {
        Ok(a) => str = a,
        Err(_) => {}
    };

    let r = Utc.datetime_from_str(&str, FORMAT);
    match r {
        Ok(t) => Ok(Option::from(t)),
        Err(e) => Err(serde::de::Error::custom(e)),
    }
}
