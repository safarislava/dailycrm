use crate::endpoint::api_error::ApiError;
use crate::model::project::project::Project;
use crate::model::project::stage::Stage;
use crate::model::task::contract::task::Task;
use crate::model::task::project::gip_confirmation::GipConfirmation;
use crate::state::AppState;
use actix_web::web::Json;
use actix_web::{HttpResponse, web};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct Body {
    confirmed: bool,
}

pub async fn patch(
    state: web::Data<AppState>,
    path: web::Path<(Uuid, i32)>,
    body: Json<Body>,
) -> Result<HttpResponse, ApiError> {
    let (project_id, position) = path.into_inner();
    let stage = Stage::new(Project::new(project_id), position);
    GipConfirmation::new(state.pool.clone(), stage, body.confirmed)
        .done()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    Ok(HttpResponse::Ok().finish())
}
