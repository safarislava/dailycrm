use crate::endpoint::auth_header::UserHeader;
use crate::model::credential::username::Username;
use crate::model::credential::valid_username::ValidUsername;
use crate::model::task::contract::task::Task;
use crate::model::task::user::username_update::UsernameUpdate;
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
    let user = match request.user() {
        Some(u) => u,
        None => return HttpResponse::Unauthorized().finish(),
    };
    let username = ValidUsername::new(Username::new(body.username.clone()));
    let task = UsernameUpdate::new(state.pool.clone(), user, username);
    match task.done().await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().body("Something went wrong"),
    }
}
