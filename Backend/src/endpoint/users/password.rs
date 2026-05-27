use crate::auth::UserIdGettable;
use crate::model::password::{Password, ValidPassword};
use crate::model::password_hash::VerifyError;
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
    let user_id = match request.user_id() {
        Some(id) => id,
        None => return HttpResponse::Unauthorized().finish(),
    };

    let valid_new_password = match ValidPassword::try_new(Password(body.new_password.clone())) {
        Ok(p) => p,
        Err(e) => return HttpResponse::UnprocessableEntity().body(e.message()),
    };

    match state.users.password_verification(user_id, &body.current_password).await {
        Ok(_) => {}
        Err(VerifyError::WrongPassword) => {
            return HttpResponse::Unauthorized().body("Wrong current password");
        }
        Err(VerifyError::Internal) => {
            return HttpResponse::InternalServerError().body("Something went wrong");
        }
    };

    let new_hash = match valid_new_password.hashed().await {
        Ok(h) => h,
        Err(_) => return HttpResponse::InternalServerError().body("Something went wrong"),
    };

    match state.users.update_password(user_id, &new_hash).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().body("Something went wrong"),
    }
}