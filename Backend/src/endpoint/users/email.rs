use crate::endpoint::auth_header::UserHeader;
use crate::model::task::contract::task::Task;
use crate::model::task::user::email_update::EmailUpdate;
use crate::state::AppState;
use actix_web::{HttpRequest, HttpResponse, Responder, web};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct UpdateEmailBody {
    email: String,
}

pub async fn patch(
    state: web::Data<AppState>,
    request: HttpRequest,
    body: web::Json<UpdateEmailBody>,
) -> impl Responder {
    let user = match request.user() {
        Some(u) => u,
        None => return HttpResponse::Unauthorized().finish(),
    };
    let task = EmailUpdate::new(state.pool.clone(), user, body.email.clone());
    match task.done().await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().body("Something went wrong"),
    }
}
