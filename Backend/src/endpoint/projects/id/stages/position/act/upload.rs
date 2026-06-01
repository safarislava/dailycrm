use crate::endpoint::api_error::ApiError;
use crate::endpoint::auth_header::UserHeader;
use crate::model::project::project::Project;
use crate::model::project::stage::Stage;
use crate::model::task::contract::task::Task;
use crate::model::task::project::logged_act_upload::LoggedActUpload;
use crate::state::AppState;
use actix_multipart::Multipart;
use actix_web::{HttpRequest, HttpResponse, web};
use futures_util::StreamExt;
use uuid::Uuid;

const MAX_FILE_SIZE: usize = 50 * 1_048_576;

async fn collect_bytes(
    field: &mut actix_multipart::Field,
    limit: usize,
) -> Result<Vec<u8>, ApiError> {
    let mut data = Vec::new();
    while let Some(chunk) = field.next().await {
        match chunk {
            Ok(bytes) => {
                data.extend_from_slice(&bytes);
                if data.len() > limit {
                    return Err(ApiError::PayloadTooLarge);
                }
            }
            Err(_) => return Err(ApiError::Internal("Failed to read upload".to_string())),
        }
    }
    Ok(data)
}

pub async fn post(
    state: web::Data<AppState>,
    request: HttpRequest,
    path: web::Path<(Uuid, i32)>,
    mut payload: Multipart,
) -> Result<HttpResponse, ApiError> {
    let user = request
        .user()
        .ok_or(ApiError::Unauthorized("Unauthorized".to_string()))?;
    let (project_id, stage_position) = path.into_inner();
    let stage = Stage::new(Project::new(project_id), stage_position);

    while let Some(item) = payload.next().await {
        let mut field =
            item.map_err(|_| ApiError::BadRequest("Invalid multipart data".to_string()))?;
        let filename = field
            .content_disposition()
            .and_then(|cd| cd.get_filename())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "act".to_string());
        let data = collect_bytes(&mut field, MAX_FILE_SIZE).await?;
        let mime_type = infer::get(&data)
            .map(|kind| kind.mime_type().to_string())
            .unwrap_or_else(|| "application/octet-stream".to_string());
        LoggedActUpload::new(
            state.pool.clone(),
            state.storage.clone(),
            stage,
            user,
            filename,
            mime_type,
            data,
        )
        .done()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;
        return Ok(HttpResponse::Created().finish());
    }
    Err(ApiError::BadRequest("No file provided".to_string()))
}