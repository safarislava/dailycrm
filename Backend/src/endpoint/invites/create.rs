use crate::auth::UserIdGettable;
use crate::model::credential::contract::contentable::Contentable;
use crate::model::task::task::Task;
use crate::model::task::user::invite_creation::InviteCreation;
use crate::model::user::user::User;
use crate::state::AppState;
use actix_web::{HttpRequest, HttpResponse, Responder, web};

pub async fn post(state: web::Data<AppState>, request: HttpRequest) -> impl Responder {
    let user_id = match request.user_id() {
        Some(id) => id,
        None => return HttpResponse::Unauthorized().finish(),
    };
    let user = User::new(state.pool.clone(), user_id);
    let invite_creation = InviteCreation::new(state.pool.clone(), user);
    match invite_creation.output().await {
        Ok(invite) => match invite.content().await {
            Ok(token) => HttpResponse::Created().json(serde_json::json!({ "token": token })),
            Err(_) => HttpResponse::InternalServerError().body("Something went wrong"),
        },
        Err(_) => HttpResponse::InternalServerError().body("Something went wrong"),
    }
}
