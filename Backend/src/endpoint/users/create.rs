use crate::model::invites::RegisterWithInviteResult;
use crate::model::password::Password;
use crate::model::username::Username;
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
    let valid_username = match Username(body.username.clone()).validated() {
        Ok(u) => u,
        Err(e) => return HttpResponse::UnprocessableEntity().body(e.message()),
    };

    let valid_password = match Password(body.password.clone()).validated() {
        Ok(p) => p,
        Err(e) => return HttpResponse::UnprocessableEntity().body(e.message()),
    };

    let password_hash = match valid_password.hashed().await {
        Ok(h) => h,
        Err(_) => return HttpResponse::InternalServerError().body("Something went wrong"),
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
