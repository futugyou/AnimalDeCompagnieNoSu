use crate::custom_error::CustomError;
use bson::oid::ObjectId;
use chrono::{DateTime, Utc};

pub mod base64convert;
pub mod custom_error;

pub const FORMAT: &'static str = "%Y %m %d %H:%M:%S%.3f %z"; 
pub const DATETIMEDEFAULT: &'static str = "1900 01 01 00:00:00.000 +0000";

pub fn getdefaultdatetime() -> DateTime<Utc> {
    DateTime::parse_from_str(DATETIMEDEFAULT, FORMAT)
        .unwrap()
        .to_utc()
}

pub fn getutcnowwithformat() -> DateTime<Utc> {
    Utc::now()
}

#[allow(non_snake_case)]
pub fn stringtoObjectId(str: &str) -> Result<bson::oid::ObjectId, CustomError> {
    Ok(ObjectId::parse_str(str)?)
}
