use crate::endpoint::api_error::ApiError;
use crate::endpoint::auth_header::AuthHeader;
use crate::model::audit::AuditAction;
use crate::model::audit::AuditedTask;
use crate::model::contract::task::Task;
use crate::model::project::stage::cost::contract::cost::Cost;
use crate::model::project::stage::cost::r#final::logged_update::LoggedFinalCostUpdate;
use crate::model::project::stage::id::StageId;
use crate::model::project::stage::invalidating_by_project_id::InvalidatingByProjectId;
use crate::state::AppState;
use actix_web::web::Json;
use actix_web::{HttpRequest, HttpResponse, web};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct UpdateCostDto {
    cost: Option<Box<dyn Cost>>,
}

pub async fn patch(
    state: web::Data<AppState>,
    request: HttpRequest,
    stage_id: StageId,
    body: Json<UpdateCostDto>,
) -> Result<HttpResponse, ApiError> {
    let user = request
        .user()
        .ok_or(ApiError::Unauthorized("Unauthorized".to_string()))?;
    let cost = body.into_inner().cost;
    let project_id = stage_id.project_id();
    let position = stage_id.position();
    AuditedTask::new(
        user,
        AuditAction::FinalCostUpdate {
            new_cost: cost.clone(),
        },
        format!("{project_id}:{position}"),
        InvalidatingByProjectId::new(
            LoggedFinalCostUpdate::new(state.pool.clone(), stage_id, user, cost),
            state.stage_cache.clone(),
            project_id,
        ),
    )
    .perform()
    .await
    .map_err(|e| ApiError::Internal(e.to_string()))?;
    Ok(HttpResponse::Ok().finish())
}
