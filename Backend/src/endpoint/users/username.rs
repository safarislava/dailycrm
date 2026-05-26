use crate::auth::user_id_from_request;
use crate::model::username::{Username, ValidUsername};
use crate::state::AppState;
use actix_web::{HttpRequest, HttpResponse, Responder, web};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct UpdateUsernameDto {
    username: String,
}

pub async fn patch(
    state: web::Data<AppState>,
    req: HttpRequest,
    body: web::Json<UpdateUsernameDto>,
) -> impl Responder {
    let user_id = match user_id_from_request(&req) {
        Some(id) => id,
        None => return HttpResponse::Unauthorized().finish(),
    };

    let valid_username = match ValidUsername::try_new(Username(body.username.clone())) {
        Ok(u) => u,
        Err(e) => return HttpResponse::UnprocessableEntity().body(e.message()),
    };

    match state
        .users
        .user_link(user_id)
        .update_username(&valid_username)
        .await
    {
        Ok(true) => HttpResponse::Ok().finish(),
        Ok(false) => HttpResponse::Conflict().body("Username already taken"),
        Err(_) => HttpResponse::InternalServerError().body("Something went wrong"),
    }
}
