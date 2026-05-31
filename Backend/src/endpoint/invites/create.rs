use crate::endpoint::auth_header::UserHeader;
use crate::model::credential::contract::contentable::Contentable;
use crate::model::task::contract::task::Task;
use crate::model::task::user::invite_creation::InviteCreation;
use crate::state::AppState;
use actix_web::{HttpRequest, HttpResponse, Responder, web};

pub async fn post(state: web::Data<AppState>, request: HttpRequest) -> impl Responder {
    let user = match request.user() {
        Some(u) => u,
        None => return HttpResponse::Unauthorized().finish(),
    };
    let invite_creation = InviteCreation::new(state.pool.clone(), user);
    match invite_creation.done().await {
        Ok(invite) => match invite.content().await {
            Ok(token) => HttpResponse::Created().json(serde_json::json!({ "token": token })),
            Err(_) => HttpResponse::InternalServerError().body("Something went wrong"),
        },
        Err(_) => HttpResponse::InternalServerError().body("Something went wrong"),
    }
}
