use crate::endpoint::api_error::ApiError;
use crate::endpoint::auth_header::UserHeader;
use crate::model::task::contract::task::Task;
use crate::model::task::user::invite_creation::InviteCreation;
use crate::model::user::contract::invite::Invite;
use crate::state::AppState;
use actix_web::{HttpRequest, HttpResponse, web};

pub async fn post(
    state: web::Data<AppState>,
    request: HttpRequest,
) -> Result<HttpResponse, ApiError> {
    let user = request
        .user()
        .ok_or(ApiError::Unauthorized("Unauthorized".to_string()))?;
    let invite = InviteCreation::new(state.pool.clone(), user)
        .done()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    Ok(HttpResponse::Created().json(serde_json::json!({ "token": invite.token() })))
}