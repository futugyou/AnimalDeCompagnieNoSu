use chrono::{DateTime, TimeZone, Utc};
use custom_error::CustomError;

pub mod config;
pub mod context;
pub mod custom_error;
pub mod serialize;
pub mod structmapper;

pub const FORMAT: &'static str = "%Y-%m-%dT%H:%M:%SZ";
pub const DATETIMEDEFAULT: &'static str = "1900-01-01T01:01:01Z";

pub fn getdefaultdatetime() -> DateTime<Utc> {
    Utc.datetime_from_str(DATETIMEDEFAULT, FORMAT).unwrap()
}

pub fn stringtoObjectId(str: &str) -> Result<bson::oid::ObjectId, CustomError> {
    Ok(bson::oid::ObjectId::with_string(str)?)
}
