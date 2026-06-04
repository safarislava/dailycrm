use crate::endpoint::api_error::ApiError;
use crate::endpoint::auth_header::UserHeader;
use crate::model::credential::contract::username::Username;
use crate::model::user::detailed_user::DetailedUser;
use crate::state::AppState;
use actix_web::{HttpRequest, HttpResponse, web};

pub async fn get(
    state: web::Data<AppState>,
    request: HttpRequest,
) -> Result<HttpResponse, ApiError> {
    let user = request
        .user()
        .ok_or(ApiError::Unauthorized("Unauthorized".to_string()))?;
    let user = DetailedUser::new(state.pool.clone(), user);
    let username = user
        .username()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    let email = user
        .email()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    let notifications_enabled = user
        .notifications_enabled()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    let roles = user
        .roles()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    match (username, email, notifications_enabled) {
        (Some(username), Some(email), Some(notifications_enabled)) => {
            let username = username
                .value()
                .map_err(|e| ApiError::Internal(e.to_string()))?;
            Ok(HttpResponse::Ok().json(serde_json::json!({
                "username": username,
                "email": email,
                "notifications_enabled": notifications_enabled,
                "roles": roles,
            })))
        }
        _ => Err(ApiError::NotFound("User not found".to_string())),
    }
}