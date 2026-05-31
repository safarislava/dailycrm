use crate::endpoint::api_error::ApiError;
use crate::endpoint::auth_header::UserHeader;
use crate::model::credential::contract::contentable::Contentable;
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
    let (username, email, notifications_enabled) = match (
        user.username().await,
        user.email().await,
        user.notifications_enabled().await,
    ) {
        (Ok(u), Ok(e), Ok(n)) => (u, e, n),
        (Err(e), _, _) | (_, Err(e), _) | (_, _, Err(e)) => {
            return Err(ApiError::Internal(e.to_string()));
        }
    };
    match (username, email, notifications_enabled) {
        (Some(username), Some(email), Some(notifications_enabled)) => {
            let username = username
                .content()
                .await
                .map_err(|e| ApiError::Internal(e.to_string()))?;
            Ok(HttpResponse::Ok().json(serde_json::json!({
                "username": username,
                "email": email,
                "notifications_enabled": notifications_enabled,
            })))
        }
        _ => Err(ApiError::NotFound("User not found".to_string())),
    }
}
