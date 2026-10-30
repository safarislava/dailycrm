use crate::endpoint::api_error::ApiError;
use crate::endpoint::auth_header::AuthHeader;
use crate::model::contract::task::Task;
use crate::model::project::stage::cost::logged_tranche_deletion::LoggedTrancheDeletion;
use crate::model::project::stage::cost::text::FinalTrancheText;
use crate::model::project::stage::id::StageId;
use crate::model::project::stage::invalidating_by_project_id::InvalidatingByProjectId;
use crate::state::AppState;
use actix_web::{HttpRequest, HttpResponse, web};
use uuid::Uuid;

pub async fn delete(
    state: web::Data<AppState>,
    request: HttpRequest,
    stage_id: StageId,
) -> Result<HttpResponse, ApiError> {
    let user = request
        .user()
        .ok_or(ApiError::Unauthorized("Unauthorized".to_string()))?;

    let tranche_id = request
        .match_info()
        .get("tranche_id")
        .and_then(|s| Uuid::parse_str(s).ok())
        .ok_or_else(|| ApiError::BadRequest("Invalid tranche id".to_string()))?;

    let project_id = stage_id.project_id();

    InvalidatingByProjectId::new(
        LoggedTrancheDeletion::new(
            state.pool.clone(),
            stage_id,
            user,
            "final",
            tranche_id,
            FinalTrancheText,
        ),
        state.stage_cache.clone(),
        project_id,
    )
    .perform()
    .await
    .map_err(ApiError::from)?;

    Ok(HttpResponse::Ok().finish())
}
