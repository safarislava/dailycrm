use crate::contract::contentable::Contentable;
use crate::model::credential::hashed_password::HashedPassword;
use crate::model::credential::password::Password;
use crate::model::credential::username::Username;
use crate::model::credential::valid_password::ValidPassword;
use crate::model::credential::valid_username::ValidUsername;
use crate::model::user::invites::RegisterWithInviteResult;
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
    let username = ValidUsername::new(Username::new(body.username.clone()));

    let hashed_password =
        HashedPassword::new(ValidPassword::new(Password::new(body.password.clone())));
    let hash = match hashed_password.content().await {
        Ok(hash) => hash,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    match state
        .invites
        .consume_and_register(body.invite_token, &username, &hash)
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
