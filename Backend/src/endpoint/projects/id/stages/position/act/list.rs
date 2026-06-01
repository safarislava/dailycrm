use crate::endpoint::api_error::ApiError;
use crate::model::credential::contract::contentable::Contentable;
use crate::model::project::acts::Acts;
use crate::model::project::contract::list::List;
use crate::model::project::detailed_act::DetailedAct;
use crate::model::project::project::Project;
use crate::model::project::stage::Stage;
use crate::state::AppState;
use actix_web::{HttpResponse, web};
use futures_util::future::try_join_all;
use uuid::Uuid;

pub async fn get(
    state: web::Data<AppState>,
    path: web::Path<(Uuid, i32)>,
) -> Result<HttpResponse, ApiError> {
    let (project_id, stage_position) = path.into_inner();
    let stage = Stage::new(Project::new(project_id), stage_position);
    let list = Acts::new(state.pool.clone(), stage)
        .items()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    let futures = list.into_iter().map(|act| {
        let detailed = DetailedAct::new(state.pool.clone(), act);
        async move { detailed.content().await }
    });
    let items = try_join_all(futures)
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    Ok(HttpResponse::Ok().json(items))
}
