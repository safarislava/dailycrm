use crate::endpoint::api_error::ApiError;
use crate::endpoint::auth_header::AuthHeader;
use crate::model::contract::task::Task;
use crate::model::project::stage::cost::logged_payment_confirmation::LoggedPaymentConfirmation;
use crate::model::project::stage::cost::text::FinalConfirmationText;
use crate::model::project::stage::id::StageId;
use crate::model::project::stage::invalidating_by_project_id::InvalidatingByProjectId;
use crate::state::AppState;
use actix_web::web::Json;
use actix_web::{HttpRequest, HttpResponse, web};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Body {
    confirmed: bool,
}

pub async fn patch(
    state: web::Data<AppState>,
    request: HttpRequest,
    stage_id: StageId,
    body: Json<Body>,
) -> Result<HttpResponse, ApiError> {
    let user = request
        .user()
        .ok_or(ApiError::Unauthorized("Unauthorized".to_string()))?;
    InvalidatingByProjectId::new(
        LoggedPaymentConfirmation::new(
            state.pool.clone(),
            stage_id,
            user,
            "final",
            body.confirmed,
            FinalConfirmationText,
        ),
        state.stage_cache.clone(),
        stage_id.project_id(),
    )
    .perform()
    .await
    .map_err(|e| ApiError::Internal(e.to_string()))?;
    Ok(HttpResponse::Ok().finish())
}
