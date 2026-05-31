use crate::endpoint::api_error::ApiError;
use crate::model::project::attachment::Attachment;
use crate::model::task::contract::task::Task;
use crate::model::task::project::attachment_removal::AttachmentRemoval;
use crate::state::AppState;
use actix_web::{HttpResponse, web};
use uuid::Uuid;

pub async fn delete(
    state: web::Data<AppState>,
    path: web::Path<(Uuid, i32, Uuid)>,
) -> Result<HttpResponse, ApiError> {
    let (_, _, act_id) = path.into_inner();
    AttachmentRemoval::new(state.pool.clone(), state.storage.clone(), Attachment::new(act_id))
        .done()
        .await
        .map_err(|_| ApiError::NotFound("Act not found".to_string()))?;
    Ok(HttpResponse::Ok().finish())
}