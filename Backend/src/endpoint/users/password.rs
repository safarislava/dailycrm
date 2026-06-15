use crate::endpoint::api_error::ApiError;
use crate::endpoint::auth_header::UserHeader;
use crate::model::credential::db_hash::DbHash;
use crate::model::credential::hash_user_verification::HashUserVerification;
use crate::model::credential::hashed_password::HashedPassword;
use crate::model::credential::raw_password::RawPassword;
use crate::model::credential::valid_password::ValidPassword;
use crate::model::task::contract::task::Task;
use crate::model::task::user::password_update::PasswordUpdate;
use crate::model::user::verification_protected_user::VerificationProtectedUser;
use crate::state::AppState;
use actix_web::{HttpRequest, HttpResponse, web};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct UpdatePasswordDto {
    current_password: String,
    new_password: String,
}

pub async fn patch(
    state: web::Data<AppState>,
    request: HttpRequest,
    body: web::Json<UpdatePasswordDto>,
) -> Result<HttpResponse, ApiError> {
    let user = request
        .user()
        .ok_or(ApiError::Unauthorized("Unauthorized".to_string()))?;
    let current_password = ValidPassword::new(RawPassword::new(body.current_password.clone()));
    let hash = DbHash::new(state.pool.clone(), user.clone());
    let verification = HashUserVerification::new(hash, current_password);
    let user = VerificationProtectedUser::new(user, verification);
    let new_password = HashedPassword::new(ValidPassword::new(RawPassword::new(body.new_password.clone())));
    PasswordUpdate::new(state.pool.clone(), Box::new(user), Box::new(new_password))
        .done()
        .await?;
    Ok(HttpResponse::Ok().finish())
}
