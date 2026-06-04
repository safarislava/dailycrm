use crate::endpoint::api_error::ApiError;
use crate::endpoint::auth_header::UserHeader;
use crate::model::project::project::Project;
use crate::model::project::stage::Stage;
use crate::model::task::contract::task::Task;
use crate::model::task::project::logged_cost_update::LoggedCostUpdate;
use crate::state::AppState;
use actix_web::web::Json;
use actix_web::{HttpRequest, HttpResponse, web};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct Body {
    cost: Option<i32>,
}

pub async fn patch(
    state: web::Data<AppState>,
    request: HttpRequest,
    path: web::Path<(Uuid, i32, i32)>,
    body: Json<Body>,
) -> Result<HttpResponse, ApiError> {
    let user = request.user().ok_or(ApiError::Unauthorized("Unauthorized".to_string()))?;
    let (project_id, parent_position, position) = path.into_inner();
    LoggedCostUpdate::new(
        state.pool.clone(),
        Stage::new_substage(Project::new(project_id), parent_position, position),
        user,
        body.cost,
    )
    .done()
    .await
    .map_err(|e| ApiError::Internal(e.to_string()))?;
    Ok(HttpResponse::Ok().finish())
}