use crate::endpoint::api_error::ApiError;
use crate::endpoint::projects::id::stages::position::comments::media::JsonCommentMedia;
use crate::model::contract::printer::Printer;
use crate::model::project::archive::comment_summaries::ArchivedCommentSummaries;
use crate::model::project::stage::id::StageId;
use crate::state::AppState;
use actix_web::{HttpResponse, web};

pub async fn get(state: web::Data<AppState>, stage_id: StageId) -> Result<HttpResponse, ApiError> {
    let mut media = JsonCommentMedia::default();
    ArchivedCommentSummaries::new(state.storage.clone(), stage_id, None)
        .print(&mut media)
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    Ok(HttpResponse::Ok().json(media))
}
