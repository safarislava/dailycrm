use crate::endpoint::auth_header::UserHeader;
use crate::model::task::contract::task::Task;
use crate::model::task::user::notifications_update::NotificationsUpdate;
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
    let user = match request.user() {
        Some(u) => u,
        None => return HttpResponse::Unauthorized().finish(),
    };
    let task = NotificationsUpdate::new(state.pool.clone(), user, body.enabled);
    match task.done().await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().body("Something went wrong"),
    }
}
