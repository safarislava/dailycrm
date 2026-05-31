use crate::endpoint::api_error::ApiError;
use crate::endpoint::auth::session_response::SessionResponse;
use crate::model::session::contract::jti_source::JtiSource;
use crate::model::session::refresh_token::RefreshToken;
use crate::model::session::signed_refresh_token::SignedRefreshToken;
use crate::model::task::contract::task::Task;
use crate::model::task::user::tokens_issuance::TokenIssuance;
use crate::model::user::jwt_protected_user::JwtProtectedUser;
use crate::state::AppState;
use actix_web::{HttpRequest, HttpResponse, web};

pub async fn post(
    state: web::Data<AppState>,
    request: HttpRequest,
) -> Result<HttpResponse, ApiError> {
    let cookie = request
        .cookie("refresh_token")
        .ok_or(ApiError::Unauthorized("No refresh token".to_string()))?;
    let jti = SignedRefreshToken::new(cookie.value().to_string())
        .jti()
        .ok_or(ApiError::Unauthorized("Invalid refresh token".to_string()))?;
    let refresh_token = RefreshToken::new(jti, Box::new(cookie.value().to_string()));
    let user = JwtProtectedUser::new(state.pool.clone(), refresh_token);
    let (access, refresh) = TokenIssuance::new(state.pool.clone(), Box::new(user))
        .done()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?
        .ok_or(ApiError::Unauthorized(
            "Token revoked or expired".to_string(),
        ))?;
    Ok(SessionResponse::new(access, refresh).response().await)
}
