use crate::model::invites::RegisterWithInviteResult;
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
    let password = body.password.clone();
    let password_hash = match actix_web::rt::task::spawn_blocking(move || {
        bcrypt::hash(password, bcrypt::DEFAULT_COST)
    })
    .await
    {
        Ok(Ok(hash)) => hash,
        _ => return HttpResponse::InternalServerError().body("Something went wrong"),
    };

    match state
        .invites
        .consume_and_register(body.invite_token, &body.username, &password_hash)
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