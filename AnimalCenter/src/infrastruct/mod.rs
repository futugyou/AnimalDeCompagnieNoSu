use serde::{de, Deserialize, Deserializer};
use std::fmt::Display;
use std::str::FromStr;

pub mod config;
pub mod context;

pub mod date_format {
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, FORMAT)
            .map_err(serde::de::Error::custom)
    }
}

pub mod date_format_option {
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &'static str = "%Y-%m-%dT%H:%M:%SZ";

    pub fn serialize<S>(date: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match date {
            Some(d) => {
                let s = format!("{}", d.format(FORMAT));
                serializer.serialize_str(&s)
            }
            None => {
                let s = "1900-01-01T01:01:01Z";
                serializer.serialize_str(s)
            }
        }
    }
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = std::collections::HashMap::<String, String>::deserialize(deserializer);
        //let s = String::deserialize(deserializer);
        let mut str = String::from("1900-01-01T01:01:01Z");
        let _ = match s {
            Ok(a) => {
                str = a.get("$date").unwrap().to_string();
            }
            Err(_) => (),
        };

        let r = Utc.datetime_from_str(&str, FORMAT);
        match r {
            Ok(t) => Ok(Option::from(t)),
            Err(e) => Err(serde::de::Error::custom(e)),
        }
    }
}

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
