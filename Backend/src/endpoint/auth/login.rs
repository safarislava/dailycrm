use crate::endpoint::api_error::ApiError;
use crate::endpoint::auth::session_response::SessionResponse;
use crate::model::credential::password::Password;
use crate::model::credential::username::Username;
use crate::model::credential::valid_password::ValidPassword;
use crate::model::credential::valid_username::ValidUsername;
use crate::model::task::contract::task::Task;
use crate::model::task::user::tokens_issuance::TokenIssuance;
use crate::model::user::contract::username_search::UsernameSearch;
use crate::model::user::protected_user::ProtectedUser;
use crate::model::user::users::Users;
use crate::state::AppState;
use actix_web::{HttpResponse, web};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginDto {
    username: String,
    password: String,
}

pub async fn post(
    state: web::Data<AppState>,
    body: web::Json<LoginDto>,
) -> Result<HttpResponse, ApiError> {
    let username = ValidUsername::new(Username::new(body.username.clone()));
    let user = Users::new(state.pool.clone())
        .found(username)
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?
        .ok_or(ApiError::Unauthorized("Invalid credentials".to_string()))?;
    let password = ValidPassword::new(Password::new(body.password.clone()));
    let user = ProtectedUser::new(state.pool.clone(), user, password);
    let (access, refresh) = TokenIssuance::new(state.pool.clone(), Box::new(user))
        .done()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?
        .ok_or(ApiError::Unauthorized("Invalid credentials".to_string()))?;
    Ok(SessionResponse::new(access, refresh).response().await)
}
