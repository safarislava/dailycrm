use crate::auth::UserIdGettable;
use crate::model::task::contract::task::Task;
use crate::model::task::user::notifications_update::NotificationsUpdate;
use crate::model::user::user::User;
use crate::state::AppState;
use actix_web::{HttpRequest, HttpResponse, Responder, web};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct UpdateNotificationsBody {
    enabled: bool,
}

pub async fn patch(
    state: web::Data<AppState>,
    request: HttpRequest,
    body: web::Json<UpdateNotificationsBody>,
) -> impl Responder {
    let user_id = match request.user_id() {
        Some(id) => id,
        None => return HttpResponse::Unauthorized().finish(),
    };
    let task = NotificationsUpdate::new(state.pool.clone(), User::new(user_id), body.enabled);
    match task.done().await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().body("Something went wrong"),
    }
}
