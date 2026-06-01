use crate::endpoint::api_error::ApiError;
use crate::model::project::comment::Comment;
use crate::model::task::contract::task::Task;
use crate::model::task::project::comment_removal::CommentRemoval;
use crate::state::AppState;
use actix_web::{HttpResponse, web};
use uuid::Uuid;

pub async fn delete(
    state: web::Data<AppState>,
    path: web::Path<(Uuid, i32, Uuid)>,
) -> Result<HttpResponse, ApiError> {
    let (_, _, comment_id) = path.into_inner();
    CommentRemoval::new(state.pool.clone(), Comment::new(comment_id))
        .done()
        .await
        .map_err(|_| ApiError::NotFound("Comment not found".to_string()))?;
    Ok(HttpResponse::Ok().finish())
}