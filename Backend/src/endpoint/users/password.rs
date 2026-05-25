use crate::auth::user_id_from_request;
use crate::model::user::{Password, PasswordHash, ValidPassword, ValidPasswordHash, VerifyError};
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
    req: HttpRequest,
    body: web::Json<UpdatePasswordDto>,
) -> impl Responder {
    let user_id = match user_id_from_request(&req) {
        Some(id) => id,
        None => return HttpResponse::Unauthorized().finish(),
    };

    let valid_new_password = match ValidPassword::try_new(Password(body.new_password.clone())) {
        Ok(p) => p,
        Err(e) => return HttpResponse::UnprocessableEntity().body(e.message()),
    };

    let stored_hash = match state.users.password_hash_by_id(user_id).await {
        Ok(Some(h)) => h,
        Ok(None) => return HttpResponse::NotFound().finish(),
        Err(_) => return HttpResponse::InternalServerError().body("Something went wrong"),
    };

    match ValidPasswordHash::try_new(stored_hash, &body.current_password).await {
        Ok(_) => {}
        Err(VerifyError::WrongPassword) => return HttpResponse::Unauthorized().body("Wrong current password"),
        Err(VerifyError::Internal) => return HttpResponse::InternalServerError().body("Something went wrong"),
    };

    let new_hash = match PasswordHash::new_from_password(valid_new_password).await {
        Ok(h) => h,
        Err(_) => return HttpResponse::InternalServerError().body("Something went wrong"),
    };

    match state.users.update_password(user_id, &new_hash).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().body("Something went wrong"),
    }
}