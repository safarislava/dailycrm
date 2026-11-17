use crate::endpoint::api_error::ApiError;
use crate::endpoint::auth_header::AuthHeader;
use crate::model::audit::AuditAction;
use crate::model::audit::AuditedTask;
use crate::model::contract::task::Task;
use crate::model::project::archive::restoration::ProjectRestoration;
use crate::model::project::id::ProjectId;
use crate::model::project::invalidating_by_project_id::InvalidatingByProjectId;
use crate::state::AppState;
use actix_web::{HttpRequest, HttpResponse, web};

pub async fn post(
    state: web::Data<AppState>,
    request: HttpRequest,
    path: web::Path<ProjectId>,
) -> Result<HttpResponse, ApiError> {
    let user = request
        .user()
        .ok_or(ApiError::Unauthorized("Unauthorized".to_string()))?;
    let project_id = path.into_inner();
    AuditedTask::new(
        user,
        AuditAction::ProjectRestore,
        project_id.id(),
        InvalidatingByProjectId::new(
            ProjectRestoration::new(state.pool.clone(), state.storage.clone(), project_id),
            state.project_cache.clone(),
            project_id,
        ),
    )
    .perform()
    .await
    .map_err(|e| ApiError::Internal(e.to_string()))?;
    Ok(HttpResponse::Ok().finish())
}
