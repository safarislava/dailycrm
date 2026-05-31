use crate::endpoint::auth_header::UserHeader;
use crate::model::credential::hashed_password::HashedPassword;
use crate::model::credential::password::Password;
use crate::model::credential::valid_password::ValidPassword;
use crate::model::task::contract::task::Task;
use crate::model::task::user::password_update::PasswordUpdate;
use crate::model::user::protected_user::ProtectedUser;
use crate::state::AppState;
use actix_web::{HttpRequest, HttpResponse, Responder, web};
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
) -> impl Responder {
    let user = match request.user() {
        Some(u) => u,
        None => return HttpResponse::Unauthorized().finish(),
    };
    let current_password = ValidPassword::new(Password::new(body.current_password.clone()));
    let new_password =
        HashedPassword::new(ValidPassword::new(Password::new(body.new_password.clone())));
    let user = ProtectedUser::new(state.pool.clone(), user, current_password);
    let task = PasswordUpdate::new(state.pool.clone(), Box::new(user), Box::new(new_password));
    match task.done().await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::Unauthorized().body("Wrong current password"),
    }
}
