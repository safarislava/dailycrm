use crate::auth::UserIdGettable;
use crate::state::AppState;
use actix_web::{HttpRequest, HttpResponse, Responder, web};

pub async fn get(state: web::Data<AppState>, request: HttpRequest) -> impl Responder {
    let user_id = match request.user_id() {
        Some(id) => id,
        None => return HttpResponse::Unauthorized().finish(),
    };

    match state.users.user(user_id).username().await {
        Ok(Some(username)) => HttpResponse::Ok().json(serde_json::json!({ "username": username })),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().body("Something went wrong"),
    }
}
