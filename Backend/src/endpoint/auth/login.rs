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
use actix_web::{HttpResponse, Responder, web};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginDto {
    username: String,
    password: String,
}

pub async fn post(state: web::Data<AppState>, body: web::Json<LoginDto>) -> impl Responder {
    let users = Users::new(state.pool.clone());
    let username = ValidUsername::new(Username::new(body.username.clone()));
    let user = match users.found(username).await {
        Ok(Some(u)) => u,
        Ok(None) => return HttpResponse::Unauthorized().body("Invalid credentials"),
        Err(_) => return HttpResponse::InternalServerError().body("Something went wrong"),
    };
    let password = ValidPassword::new(Password::new(body.password.clone()));
    let user = ProtectedUser::new(state.pool.clone(), user, password);
    let task = TokenIssuance::new(state.pool.clone(), Box::new(user));
    match task.done().await {
        Ok(Some((access, refresh))) => SessionResponse::new(access, refresh).response().await,
        Ok(None) => HttpResponse::Unauthorized().body("Invalid credentials"),
        Err(_) => HttpResponse::InternalServerError().body("Something went wrong"),
    }
}
