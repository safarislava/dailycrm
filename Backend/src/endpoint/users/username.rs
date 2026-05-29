use crate::auth::UserIdGettable;
use crate::model::credential::username::Username;
use crate::model::credential::valid_username::ValidUsername;
use crate::model::task::task::Task;
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
    let user_id = match request.user_id() {
        Some(id) => id,
        None => return HttpResponse::Unauthorized().finish(),
    };

    let username = ValidUsername::new(Username::new(body.username.clone()));
    let user = state.users.user(user_id);
    let task = UsernameUpdate::new(state.pool.clone(), user, username);

    match task.output().await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().body("Something went wrong"),
    }
}
