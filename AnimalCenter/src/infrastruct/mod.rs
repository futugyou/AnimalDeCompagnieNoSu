use chrono::{DateTime, TimeZone, Utc};

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
