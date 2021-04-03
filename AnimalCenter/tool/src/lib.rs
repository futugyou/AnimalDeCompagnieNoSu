use crate::custom_error::CustomError;
use chrono::{DateTime, Duration, NaiveDateTime, TimeZone, Utc};

pub mod base64convert;
pub mod custom_error;

pub const FORMAT: &'static str = "%Y-%m-%dT%H:%M:%SZ";
pub const NAIVE_FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";
pub const DATETIMEDEFAULT: &'static str = "1900-01-01T01:01:01Z";

pub fn getdefaultnaivedatetime() -> NaiveDateTime {
    (Utc::now() + Duration::hours(0)).naive_utc()
}
pub fn getdefaultdatetime() -> DateTime<Utc> {
    Utc.datetime_from_str(DATETIMEDEFAULT, FORMAT).unwrap()
}

pub fn getutcnowwithformat() -> DateTime<Utc> {
    let date_string = Utc::now().format(FORMAT).to_string();
    let date = Utc.datetime_from_str(&date_string, FORMAT).unwrap();
    date
}

#[allow(non_snake_case)]
pub fn stringtoObjectId(str: &str) -> Result<bson::oid::ObjectId, CustomError> {
    Ok(bson::oid::ObjectId::with_string(str)?)
}
