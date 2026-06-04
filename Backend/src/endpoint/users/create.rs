use crate::endpoint::api_error::ApiError;
use crate::model::credential::hashed_password::HashedPassword;
use crate::model::credential::raw_password::RawPassword;
use crate::model::credential::raw_username::RawUsername;
use crate::model::credential::valid_password::ValidPassword;
use crate::model::credential::valid_username::ValidUsername;
use crate::model::task::contract::task::Task;
use crate::model::task::user::invite_consumption::{InviteConsumption, InviteStatus};
use crate::model::user::invite::InviteCode;
use crate::state::AppState;
use actix_web::{HttpResponse, web};
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct CreateUserDto {
    username: String,
    password: String,
    invite_token: Uuid,
    email: String,
}

pub async fn create(
    state: web::Data<AppState>,
    body: web::Json<CreateUserDto>,
) -> Result<HttpResponse, ApiError> {
    let invite = InviteCode::new(body.invite_token);
    let username = ValidUsername::new(RawUsername::new(body.username.clone()));
    let password = HashedPassword::new(ValidPassword::new(RawPassword::new(body.password.clone())));
    let result = InviteConsumption::new(
        state.pool.clone(),
        invite,
        username,
        password,
        body.email.clone(),
    )
    .done()
    .await
    .map_err(|e| ApiError::Internal(e.to_string()))?;
    match result {
        InviteStatus::Ok => Ok(HttpResponse::Created().finish()),
        InviteStatus::InvalidInvite => {
            Err(ApiError::Forbidden("Invalid or expired invite".to_string()))
        }
        InviteStatus::UserExists => Err(ApiError::Conflict("User already exists".to_string())),
    }
}