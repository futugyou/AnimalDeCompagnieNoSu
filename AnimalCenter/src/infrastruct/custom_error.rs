use actix_web::ResponseError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomError {
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
