use crate::endpoint::api_error::ApiError;
use crate::endpoint::auth_header::AuthHeader;
use crate::model::contract::task::Task;
use crate::model::project::stage::cost::logged_tranche_creation::LoggedTrancheCreation;
use crate::model::project::stage::cost::text::FinalTrancheText;
use crate::model::project::stage::cost::valid_cost::ValidCost;
use crate::model::project::stage::id::StageId;
use crate::model::project::stage::invalidating_by_project_id::InvalidatingByProjectId;
use crate::state::AppState;
use actix_web::web::Json;
use actix_web::{HttpRequest, HttpResponse, web};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateTrancheDto {
    pub amount: ValidCost,
}

pub async fn post(
    state: web::Data<AppState>,
    request: HttpRequest,
    stage_id: StageId,
    body: Json<CreateTrancheDto>,
) -> Result<HttpResponse, ApiError> {
    let user = request
        .user()
        .ok_or(ApiError::Unauthorized("Unauthorized".to_string()))?;
    let amount = body.into_inner().amount;
    let project_id = stage_id.project_id();

    let id = InvalidatingByProjectId::new(
        LoggedTrancheCreation::new(
            state.pool.clone(),
            stage_id,
            user,
            "final",
            amount,
            FinalTrancheText,
        ),
        state.stage_cache.clone(),
        project_id,
    )
    .perform()
    .await
    .map_err(ApiError::from)?;

    Ok(HttpResponse::Created().json(serde_json::json!({ "id": id })))
}
