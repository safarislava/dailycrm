use crate::endpoint::auth::session_response::SessionResponse;
use crate::model::credential::password::Password;
use crate::model::credential::valid_password::ValidPassword;
use crate::model::task::task::Task;
use crate::model::task::user::tokens_issuance::TokenIssuance;
use crate::model::user::protected_user::ProtectedUser;
use crate::state::AppState;
use actix_web::{HttpResponse, Responder, web};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginDto {
    username: String,
    password: String,
}

pub async fn post(state: web::Data<AppState>, body: web::Json<LoginDto>) -> impl Responder {
    let user = match state.users.with_username(&body.username).await {
        Ok(Some(u)) => u,
        Ok(None) => return HttpResponse::Unauthorized().body("Invalid credentials"),
        Err(_) => return HttpResponse::InternalServerError().body("Something went wrong"),
    };
    let password = ValidPassword::new(Password::new(body.password.clone()));
    let user = ProtectedUser::new(state.pool.clone(), user, password);
    let task = TokenIssuance::new(state.pool.clone(), Box::new(user));
    match task.output().await {
        Ok(Some((access, refresh))) => SessionResponse::new(access, refresh).response().await,
        Ok(None) => HttpResponse::Unauthorized().body("Invalid credentials"),
        Err(_) => HttpResponse::InternalServerError().body("Something went wrong"),
    }
}
