use crate::auth::UserIdGettable;
use crate::model::password::Password;
use crate::state::AppState;
use actix_web::{HttpRequest, HttpResponse, Responder, web};
use serde::Deserialize;
use crate::model::hash_verification::VerificationError;
use crate::model::valid_password::ValidPassword;

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
    let user_id = match request.user_id() {
        Some(id) => id,
        None => return HttpResponse::Unauthorized().finish(),
    };

    let current_password = ValidPassword::new(Password::new(body.current_password.clone()));
    let new_password = ValidPassword::new(Password::new(body.new_password.clone()));

    let user = state.users.user(user_id).confirmed(current_password);
    match user.update_password(new_password).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(VerificationError::WrongPassword) => HttpResponse::Unauthorized().body("Wrong current password"),
        Err(VerificationError::Internal) => HttpResponse::InternalServerError().body("Something went wrong"),
    }
}
