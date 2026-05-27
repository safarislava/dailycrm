use crate::auth::UserIdGettable;
use crate::state::AppState;
use actix_web::{HttpRequest, HttpResponse, Responder, web};

pub async fn post(state: web::Data<AppState>, request: HttpRequest) -> impl Responder {
    let user_id = match request.user_id() {
        Some(id) => id,
        None => return HttpResponse::Unauthorized().finish(),
    };

    match state.invites.create(user_id).await {
        Ok(token) => HttpResponse::Created().json(serde_json::json!({ "token": token })),
        Err(_) => HttpResponse::InternalServerError().body("Something went wrong"),
    }
}
