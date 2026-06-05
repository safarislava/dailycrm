use crate::endpoint::api_error::ApiError;
use crate::endpoint::auth_header::UserHeader;
use crate::model::project::contract::json::Json;
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
    let data = DetailedUser::new(state.pool.clone(), user)
        .json()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    Ok(HttpResponse::Ok().json(data))
}