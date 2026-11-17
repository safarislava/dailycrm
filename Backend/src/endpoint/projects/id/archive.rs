use crate::endpoint::api_error::ApiError;
use crate::endpoint::auth_header::AuthHeader;
use crate::model::audit::AuditAction;
use crate::model::audit::AuditedTask;
use crate::model::contract::task::Task;
use crate::model::project::archive::archivation::ProjectArchivation;
use crate::model::project::id::ProjectId;
use crate::model::project::invalidating_by_project_id::InvalidatingByProjectId;
use crate::state::AppState;
use actix_web::{HttpRequest, HttpResponse, web};

pub async fn post(
    state: web::Data<AppState>,
    request: HttpRequest,
    project_id: ProjectId,
) -> Result<HttpResponse, ApiError> {
    let user = request
        .user()
        .ok_or(ApiError::Unauthorized("Unauthorized".to_string()))?;
    AuditedTask::new(
        user,
        AuditAction::ProjectArchive,
        project_id.id(),
        InvalidatingByProjectId::new(
            ProjectArchivation::new(state.pool.clone(), state.storage.clone(), project_id),
            state.project_cache.clone(),
            project_id,
        ),
    )
    .perform()
    .await
    .map_err(|e| ApiError::Internal(e.to_string()))?;
    Ok(HttpResponse::Ok().finish())
}
