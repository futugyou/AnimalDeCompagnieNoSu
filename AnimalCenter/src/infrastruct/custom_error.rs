use actix_web::ResponseError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomError {
    /// SerializeError, 10000 ~ 19999
    /// ValidateError, 20000 ~ 29999
    /// ConfigurationError, 30000 ~ 39999
    /// MongodbError, 40000 ~ 49999
    /// RabbitmqError, 50000 ~ 59999
    /// HttpError, 60000 ~ 69999
    /// BusinessError, 70000 ~ 79999
    code: String,
    message: String,
    error_kind: CustomErrorKind,
}

impl CustomError {
    pub fn new(code: String, message: String, error_kind: CustomErrorKind) -> Self {
        Self {
            code,
            message,
            error_kind,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CustomErrorKind {
    SerializeError,
    ValidateError,
    ConfigurationError,
    MongodbError,
    RabbitmqError,
    HttpError,
    BusinessError,
}
impl std::error::Error for CustomError {}
impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({})", self)
    }
}

impl ResponseError for CustomError {
    fn error_response(&self) -> actix_web::HttpResponse {
        actix_web::HttpResponse::Ok().json(self)
    }
}

impl std::convert::From<bson::oid::Error> for CustomError {
    fn from(error: bson::oid::Error) -> Self {
        Self {
            code: "10001".to_owned(),
            message: error.to_string(),
            error_kind: CustomErrorKind::SerializeError,
        }
    }
}

impl std::convert::From<bson::de::Error> for CustomError {
    fn from(error: bson::de::Error) -> Self {
        Self {
            code: "10002".to_owned(),
            message: error.to_string(),
            error_kind: CustomErrorKind::SerializeError,
        }
    }
}

impl serde::de::Error for CustomError {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Self {
            code: "10003".to_owned(),
            message: msg.to_string(),
            error_kind: CustomErrorKind::SerializeError,
        }
    }
}

impl serde::ser::Error for CustomError {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Self {
            code: "10004".to_owned(),
            message: msg.to_string(),
            error_kind: CustomErrorKind::SerializeError,
        }
    }
}

impl std::convert::From<serde_json::Error> for CustomError {
    fn from(error: serde_json::Error) -> Self {
        Self {
            code: "10006".to_owned(),
            message: error.to_string(),
            error_kind: CustomErrorKind::SerializeError,
        }
    }
}

impl std::convert::From<validator::ValidationErrors> for CustomError {
    fn from(error: validator::ValidationErrors) -> Self {
        Self {
            code: "20001".to_owned(),
            message: error.to_string(),
            error_kind: CustomErrorKind::ValidateError,
        }
    }
}

impl std::convert::From<config::ConfigError> for CustomError {
    fn from(error: config::ConfigError) -> Self {
        Self {
            code: "30001".to_owned(),
            message: error.to_string(),
            error_kind: CustomErrorKind::ConfigurationError,
        }
    }
}

impl std::convert::From<mongodb::error::Error> for CustomError {
    fn from(error: mongodb::error::Error) -> Self {
        Self {
            code: "40001".to_owned(),
            message: error.to_string(),
            error_kind: CustomErrorKind::MongodbError,
        }
    }
}

impl std::convert::From<lapin::Error> for CustomError {
    fn from(error: lapin::Error) -> Self {
        Self {
            code: "50001".to_owned(),
            message: error.to_string(),
            error_kind: CustomErrorKind::RabbitmqError,
        }
    }
}
