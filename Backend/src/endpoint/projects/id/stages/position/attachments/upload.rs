use crate::endpoint::api_error::ApiError;
use crate::endpoint::auth_header::AuthHeader;
use crate::endpoint::streamed_upload::create_streamed_file;
use crate::model::audit::AuditAction;
use crate::model::audit::AuditedTask;
use crate::model::contract::task::Task;
use crate::model::project::contract::file::File;
use crate::model::project::stage::attachment::logged_upload::LoggedAttachmentUpload;
use crate::model::project::stage::id::StageId;
use crate::state::AppState;
use actix_multipart::Multipart;
use actix_web::{HttpRequest, HttpResponse, web};
use futures_util::StreamExt;

const MAX_FILE_SIZE: usize = 50 * 1_048_576;

pub async fn post(
    state: web::Data<AppState>,
    request: HttpRequest,
    stage_id: StageId,
    mut payload: Multipart,
) -> Result<HttpResponse, ApiError> {
    let user = request
        .user()
        .ok_or(ApiError::Unauthorized("Unauthorized".to_string()))?;
    let field = payload
        .next()
        .await
        .ok_or(ApiError::BadRequest("No file provided".to_string()))?
        .map_err(|_| ApiError::BadRequest("Invalid multipart data".to_string()))?;
    let file = create_streamed_file(field, "file", MAX_FILE_SIZE).await?;
    let filename = file.name().to_string();
    let project_id = stage_id.project_id();
    let position = stage_id.position();
    let id = AuditedTask::new(
        user,
        AuditAction::AttachmentUpload { filename },
        format!("{project_id}:{position}"),
        LoggedAttachmentUpload::new(
            state.pool.clone(),
            state.storage.clone(),
            stage_id,
            user,
            file,
        ),
    )
    .perform()
    .await
    .map_err(|e| {
        if let Some(ApiError::PayloadTooLarge) = e.downcast_ref::<ApiError>() {
            return ApiError::PayloadTooLarge;
        }
        tracing::error!(error = %e, "Attachment upload task failed");
        ApiError::Internal(e.to_string())
    })?;
    Ok(HttpResponse::Created().json(serde_json::json!({ "id": id })))
}
