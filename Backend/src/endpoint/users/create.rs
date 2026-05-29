use crate::model::credential::password::Password;
use crate::model::credential::username::Username;
use crate::model::credential::valid_password::ValidPassword;
use crate::model::credential::valid_username::ValidUsername;
use crate::model::task::task::Task;
use crate::model::task::user::invite_consumption::{InviteConsumption, InviteStatus};
use crate::model::user::invite::Invite;
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
    let invite = Invite::new(body.invite_token);
    let username = ValidUsername::new(Username::new(body.username.clone()));
    let password = ValidPassword::new(Password::new(body.password.clone()));
    let invite_consumption = InviteConsumption::new(state.pool.clone(), invite, username, password);
    match invite_consumption.output().await {
        Ok(InviteStatus::Ok) => HttpResponse::Created().finish(),
        Ok(InviteStatus::InvalidInvite) => {
            HttpResponse::Forbidden().body("Invalid or expired invite")
        }
        Ok(InviteStatus::UserExists) => HttpResponse::Conflict().body("User already exists"),
        Err(_) => HttpResponse::InternalServerError().body("Something went wrong"),
    }
}
