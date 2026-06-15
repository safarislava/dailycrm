use crate::common::BoxError;
use crate::model::credential::contract::hash_verification::VerificationError;
use crate::model::credential::contract::password::PasswordError;
use crate::model::credential::contract::username::UsernameError;
use actix_web::HttpResponse;
use actix_web::http::StatusCode;
use std::fmt;

pub enum ApiError {
    BadRequest(String),
    Unauthorized(String),
    Forbidden(String),
    NotFound(String),
    Conflict(String),
    PayloadTooLarge,
    Internal(String),
}

impl fmt::Debug for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            ApiError::BadRequest(msg) => msg,
            ApiError::Unauthorized(msg) => msg,
            ApiError::Forbidden(msg) => msg,
            ApiError::NotFound(msg) => msg,
            ApiError::Conflict(msg) => msg,
            ApiError::PayloadTooLarge => "Payload too large",
            ApiError::Internal(msg) => msg,
        })
    }
}

impl std::error::Error for ApiError {}

impl From<BoxError> for ApiError {
    fn from(error: BoxError) -> Self {
        let mut cause: Option<&(dyn std::error::Error + 'static)> = Some(error.as_ref());
        while let Some(current) = cause {
            if current.is::<UsernameError>() || current.is::<PasswordError>() {
                return ApiError::BadRequest(error.to_string());
            }
            if let Some(VerificationError::Wrong) = current.downcast_ref::<VerificationError>() {
                return ApiError::Unauthorized(error.to_string());
            }
            cause = current.source();
        }
        ApiError::Internal(error.to_string())
    }
}

impl actix_web::ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ApiError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            ApiError::Forbidden(_) => StatusCode::FORBIDDEN,
            ApiError::NotFound(_) => StatusCode::NOT_FOUND,
            ApiError::Conflict(_) => StatusCode::CONFLICT,
            ApiError::PayloadTooLarge => StatusCode::PAYLOAD_TOO_LARGE,
            ApiError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).body(self.to_string())
    }
}
