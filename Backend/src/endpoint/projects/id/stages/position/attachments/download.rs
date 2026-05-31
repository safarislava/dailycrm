use crate::endpoint::api_error::ApiError;
use crate::model::project::attachment::Attachment;
use crate::model::task::contract::task::Task;
use crate::model::task::project::attachment_download::AttachmentDownload;
use crate::state::AppState;
use actix_web::{HttpResponse, web};
use uuid::Uuid;

pub async fn get(
    state: web::Data<AppState>,
    path: web::Path<(Uuid, i32, Uuid)>,
) -> Result<HttpResponse, ApiError> {
    let (_, _, attachment_id) = path.into_inner();
    let (stream, content_length, content_type, content_disposition) = AttachmentDownload::new(
        state.pool.clone(),
        state.storage.clone(),
        Attachment::new(attachment_id),
    )
    .done()
    .await
    .map_err(|_| ApiError::NotFound("Attachment not found".to_string()))?;
    Ok(HttpResponse::Ok()
        .content_type(content_type)
        .insert_header(("Content-Disposition", content_disposition))
        .insert_header(("Content-Length", content_length.to_string()))
        .streaming(stream))
}
