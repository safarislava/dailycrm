use crate::endpoint::api_error::ApiError;
use crate::model::contract::task::Task;
use crate::model::project::archive::attachment_download::ArchivedAttachmentDownload;
use crate::model::project::id::ProjectId;
use crate::model::project::stage::attachment::id::AttachmentId;
use crate::state::AppState;
use actix_web::{HttpRequest, HttpResponse, web};
use uuid::Uuid;

pub async fn get(state: web::Data<AppState>, req: HttpRequest) -> Result<HttpResponse, ApiError> {
    let project_id_str = req
        .match_info()
        .get("project_id")
        .ok_or_else(|| ApiError::BadRequest("Missing project_id".to_string()))?;
    let project_uuid = Uuid::parse_str(project_id_str)
        .map_err(|_| ApiError::BadRequest("Invalid project_id".to_string()))?;

    let file_id_str = req
        .match_info()
        .get("attachment_id")
        .or_else(|| req.match_info().get("act_id"))
        .ok_or_else(|| ApiError::BadRequest("Missing attachment_id".to_string()))?;
    let file_uuid = Uuid::parse_str(file_id_str)
        .map_err(|_| ApiError::BadRequest("Invalid attachment_id".to_string()))?;

    let (stream, content_length, content_type, content_disposition) =
        ArchivedAttachmentDownload::new(
            state.storage.clone(),
            ProjectId::new(project_uuid),
            AttachmentId::new(file_uuid),
        )
        .perform()
        .await
        .map_err(|_| ApiError::NotFound("File not found in archive".to_string()))?;

    Ok(HttpResponse::Ok()
        .content_type(content_type)
        .insert_header(("Content-Disposition", content_disposition))
        .insert_header(("Content-Length", content_length.to_string()))
        .streaming(stream))
}
