use crate::endpoint::api_error::ApiError;
use crate::endpoint::auth::session_response::SessionResponse;
use crate::model::credential::raw_password::RawPassword;
use crate::model::credential::raw_username::RawUsername;
use crate::model::credential::valid_password::ValidPassword;
use crate::model::credential::valid_username::ValidUsername;
use crate::model::task::contract::task::Task;
use crate::model::task::user::tokens_issuance::TokenIssuance;
use crate::model::user::contract::username_search::UsernameSearch;
use crate::model::user::users::Users;
use crate::model::user::verification_protected_user::VerificationProtectedUser;
use crate::state::AppState;
use actix_web::{HttpResponse, web};
use serde::Deserialize;
use crate::model::credential::db_hash::DbHash;
use crate::model::credential::hash_user_verification::HashUserVerification;

#[derive(Deserialize)]
pub struct LoginDto {
    username: String,
    password: String,
}

pub async fn post(
    state: web::Data<AppState>,
    body: web::Json<LoginDto>,
) -> Result<HttpResponse, ApiError> {
    let username = ValidUsername::new(RawUsername::new(body.username.clone()));
    let user = Users::new(state.pool.clone())
        .found(username)
        .await?
        .ok_or(ApiError::NotFound("User not found".to_string()))?;
    let password = ValidPassword::new(RawPassword::new(body.password.clone()));
    let hash = DbHash::new(state.pool.clone(), user.clone());
    let verification = HashUserVerification::new(hash, password);
    let user = VerificationProtectedUser::new(user, verification);
    let (access, refresh) = TokenIssuance::new(state.pool.clone(), Box::new(user))
        .done()
        .await?
        .ok_or(ApiError::Unauthorized("Invalid credentials".to_string()))?;
    Ok(SessionResponse::new(access, refresh).response().await)
}
