use crate::endpoint::api_error::ApiError;
use crate::model::credential::contract::contentable::Contentable;
use crate::model::project::contract::list::List;
use crate::model::project::deadlines::Deadlines;
use crate::state::AppState;
use actix_web::{HttpResponse, web};
use futures_util::future::try_join_all;

pub async fn get(state: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    let summaries = Deadlines::new(state.pool.clone())
        .items()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    let futures = summaries
        .into_iter()
        .map(|summary| async move { summary.content().await });
    let data = try_join_all(futures)
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    Ok(HttpResponse::Ok().json(data))
}
