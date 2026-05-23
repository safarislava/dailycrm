use crate::auth::user_id_from_request;
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

    let stored_hash = match state.users.password_hash_by_id(user_id).await {
        Ok(Some(h)) => h,
        Ok(None) => return HttpResponse::NotFound().finish(),
        Err(_) => return HttpResponse::InternalServerError().body("Something went wrong"),
    };

    let current = body.current_password.clone();
    let valid = match actix_web::rt::task::spawn_blocking(move || {
        bcrypt::verify(current, &stored_hash)
    })
    .await
    {
        Ok(Ok(v)) => v,
        _ => return HttpResponse::InternalServerError().body("Something went wrong"),
    };

    if !valid {
        return HttpResponse::Unauthorized().body("Wrong current password");
    }

    let new_password = body.new_password.clone();
    let new_hash = match actix_web::rt::task::spawn_blocking(move || {
        bcrypt::hash(new_password, bcrypt::DEFAULT_COST)
    })
    .await
    {
        Ok(Ok(h)) => h,
        _ => return HttpResponse::InternalServerError().body("Something went wrong"),
    };

    match state.users.update_password(user_id, &new_hash).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().body("Something went wrong"),
    }
}