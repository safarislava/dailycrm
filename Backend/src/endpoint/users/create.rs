use crate::model::invites::RegisterWithInviteResult;
use crate::model::user::{HashPassword, Password, Username, ValidPassword, ValidUsername};
use crate::state::AppState;
use actix_web::{HttpResponse, Responder, web};
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct CreateUserDto {
    username: String,
    password: String,
    invite_token: Uuid,
}

pub async fn create(state: web::Data<AppState>, body: web::Json<CreateUserDto>) -> impl Responder {
    let valid_username = match ValidUsername::try_new(Username(body.username.clone())) {
        Ok(u) => u,
        Err(e) => return HttpResponse::UnprocessableEntity().body(e.message()),
    };

    let valid_password = match ValidPassword::try_new(Password(body.password.clone())) {
        Ok(p) => p,
        Err(e) => return HttpResponse::UnprocessableEntity().body(e.message()),
    };

    let password_hash = match HashPassword::new(valid_password).hash().await {
        Some(h) => h,
        None => return HttpResponse::InternalServerError().body("Something went wrong"),
    };

    match state
        .invites
        .consume_and_register(body.invite_token, &valid_username, &password_hash)
        .await
    {
        Ok(RegisterWithInviteResult::Ok) => HttpResponse::Created().finish(),
        Ok(RegisterWithInviteResult::InvalidInvite) => {
            HttpResponse::Forbidden().body("Invalid or expired invite")
        }
        Ok(RegisterWithInviteResult::UserExists) => {
            HttpResponse::Conflict().body("User already exists")
        }
        Err(_) => HttpResponse::InternalServerError().body("Something went wrong"),
    }
}