use crate::endpoint::api_error::ApiError;
use crate::endpoint::auth_header::AuthHeader;
use crate::model::audit::AuditAction;
use crate::model::audit::AuditedTask;
use crate::model::contract::task::Task;
use crate::model::project::stage::cost::logged_payment_cost_update::LoggedPaymentCostUpdate;
use crate::model::project::stage::cost::text::FinalCostText;
use crate::model::project::stage::cost::valid_cost::ValidCost;
use crate::model::project::stage::id::StageId;
use crate::model::project::stage::invalidating_by_project_id::InvalidatingByProjectId;
use crate::state::AppState;
use actix_web::web::Json;
use actix_web::{HttpRequest, HttpResponse, web};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct UpdateCostDto {
    cost: Option<ValidCost>,
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
        AuditAction::CostUpdate {
            kind: "final",
            new_cost: cost,
        },
        format!("{project_id}:{position}"),
        InvalidatingByProjectId::new(
            LoggedPaymentCostUpdate::new(
                state.pool.clone(),
                stage_id,
                user,
                "final",
                cost,
                FinalCostText,
            ),
            state.stage_cache.clone(),
            project_id,
        ),
    )
    .perform()
    .await
    .map_err(ApiError::from)?;
    Ok(HttpResponse::Ok().finish())
}
