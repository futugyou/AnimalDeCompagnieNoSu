use actix_web::ResponseError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomError {
    /// ValidateError 10000 ~ 19999
    /// BusinessError 20000 ~ 29999
    /// NetworkError 30000 ~ 39999
    /// MiddlewareError 40000 ~ 49999
    /// SerializeError 50000 ~ 59999
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
    MiddlewareError,
    NetworkError,
    SerializeError,
    BusinessError,
    ValidateError,
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

impl std::convert::From<mongodb::error::Error> for CustomError {
    fn from(error: mongodb::error::Error) -> Self {
        Self {
            code: "40001".to_owned(),
            message: error.to_string(),
            error_kind: CustomErrorKind::MiddlewareError,
        }
    }
}
impl std::convert::From<config::ConfigError> for CustomError {
    fn from(error: config::ConfigError) -> Self {
        Self {
            code: "40000".to_owned(),
            message: error.to_string(),
            error_kind: CustomErrorKind::MiddlewareError,
        }
    }
}

impl std::convert::From<bson::oid::Error> for CustomError {
    fn from(error: bson::oid::Error) -> Self {
        Self {
            code: "10001".to_owned(),
            message: error.to_string(),
            error_kind: CustomErrorKind::ValidateError,
        }
    }
}

impl std::convert::From<bson::de::Error> for CustomError {
    fn from(error: bson::de::Error) -> Self {
        Self {
            code: "40002".to_owned(),
            message: error.to_string(),
            error_kind: CustomErrorKind::MiddlewareError,
        }
    }
}
