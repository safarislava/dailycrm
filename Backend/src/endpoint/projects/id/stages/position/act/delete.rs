use crate::endpoint::api_error::ApiError;
use crate::endpoint::auth_header::UserHeader;
use crate::model::project::attachment::Attachment;
use crate::model::task::contract::task::Task;
use crate::model::task::project::logged_attachment_removal::LoggedAttachmentRemoval;
use crate::state::AppState;
use actix_web::{HttpRequest, HttpResponse, web};
use uuid::Uuid;

pub async fn delete(
    state: web::Data<AppState>,
    request: HttpRequest,
    path: web::Path<(Uuid, i32, Uuid)>,
) -> Result<HttpResponse, ApiError> {
    let user = request
        .user()
        .ok_or(ApiError::Unauthorized("Unauthorized".to_string()))?;
    let (_, _, act_id) = path.into_inner();
    LoggedAttachmentRemoval::new(
        state.pool.clone(),
        state.storage.clone(),
        Attachment::new(act_id),
        user,
    )
    .done()
    .await
    .map_err(|_| ApiError::NotFound("Act not found".to_string()))?;
    Ok(HttpResponse::Ok().finish())
}