use crate::auth::UserIdGettable;
use crate::model::username::Username;
use crate::state::AppState;
use actix_web::{HttpRequest, HttpResponse, Responder, web};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct UpdateUsernameDto {
    username: String,
}

pub async fn patch(
    state: web::Data<AppState>,
    request: HttpRequest,
    body: web::Json<UpdateUsernameDto>,
) -> impl Responder {
    let user_id = match request.user_id() {
        Some(id) => id,
        None => return HttpResponse::Unauthorized().finish(),
    };

    let valid_username = match Username(body.username.clone()).validated() {
        Ok(u) => u,
        Err(e) => return HttpResponse::UnprocessableEntity().body(e.message()),
    };

    match state.users.update_username(user_id, &valid_username).await {
        Ok(true) => HttpResponse::Ok().finish(),
        Ok(false) => HttpResponse::Conflict().body("Username already taken"),
        Err(_) => HttpResponse::InternalServerError().body("Something went wrong"),
    }
}
