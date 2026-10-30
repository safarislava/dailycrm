use crate::model::contract::box_error::BoxError;
use crate::model::credential::contract::password::PasswordError;
use crate::model::credential::contract::username::UsernameError;
use crate::model::project::stage::cost::tranche::TrancheError;
use crate::model::project::stage::cost::valid_cost::CostError;
use crate::model::user::contract::user_verification::VerificationError;
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

impl From<CostError> for ApiError {
    fn from(error: CostError) -> Self {
        ApiError::BadRequest(error.to_string())
    }
}

impl From<BoxError> for ApiError {
    fn from(error: BoxError) -> Self {
        let mut cause: Option<&(dyn std::error::Error + 'static)> = Some(error.as_ref());
        while let Some(current) = cause {
            if current.is::<UsernameError>()
                || current.is::<PasswordError>()
                || current.is::<CostError>()
                || current.is::<TrancheError>()
            {
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
        let status = self.status_code();
        if status.is_server_error() {
            tracing::error!(status = status.as_u16(), error = %self, "HTTP Server Error");
        } else if status.is_client_error() {
            tracing::warn!(status = status.as_u16(), error = %self, "HTTP Client Error");
        }
        HttpResponse::build(status).json(self.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::ResponseError;

    #[test]
    fn maps_status_codes_and_responses_correctly() {
        assert_eq!(
            ApiError::BadRequest("invalid".to_string()).status_code(),
            StatusCode::BAD_REQUEST
        );
        assert_eq!(
            ApiError::Unauthorized("auth error".to_string()).status_code(),
            StatusCode::UNAUTHORIZED
        );
        assert_eq!(
            ApiError::Forbidden("access denied".to_string()).status_code(),
            StatusCode::FORBIDDEN
        );
        assert_eq!(
            ApiError::NotFound("not found".to_string()).status_code(),
            StatusCode::NOT_FOUND
        );
        assert_eq!(
            ApiError::Conflict("exists".to_string()).status_code(),
            StatusCode::CONFLICT
        );
        assert_eq!(
            ApiError::PayloadTooLarge.status_code(),
            StatusCode::PAYLOAD_TOO_LARGE
        );
        assert_eq!(
            ApiError::Internal("db crash".to_string()).status_code(),
            StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    #[test]
    fn converts_domain_username_and_password_errors_to_bad_request() {
        let err: BoxError = Box::new(UsernameError::TooShort);
        let api_err = ApiError::from(err);
        assert_eq!(api_err.status_code(), StatusCode::BAD_REQUEST);

        let pass_err: BoxError = Box::new(PasswordError::TooShort);
        let api_err2 = ApiError::from(pass_err);
        assert_eq!(api_err2.status_code(), StatusCode::BAD_REQUEST);

        let cost_err: BoxError = Box::new(CostError::TooManyDecimalPlaces);
        let api_err3 = ApiError::from(cost_err);
        assert_eq!(api_err3.status_code(), StatusCode::BAD_REQUEST);
    }

    #[test]
    fn converts_verification_wrong_error_to_unauthorized() {
        let err: BoxError = Box::new(VerificationError::Wrong);
        let api_err = ApiError::from(err);
        assert_eq!(api_err.status_code(), StatusCode::UNAUTHORIZED);
    }
}
