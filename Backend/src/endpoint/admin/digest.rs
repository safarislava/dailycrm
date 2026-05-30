use crate::model::task::contract::task::Task;
use crate::model::task::notification::deadline_digest_notification::DeadlineDigestNotification;
use crate::state::AppState;
use actix_web::{HttpResponse, Responder, web};

pub async fn post(state: web::Data<AppState>) -> impl Responder {
    match DeadlineDigestNotification::new(state.pool.clone(), state.mailer.clone())
        .done()
        .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
