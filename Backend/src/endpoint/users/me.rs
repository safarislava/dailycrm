use crate::auth::user_id_from_request;
use crate::state::AppState;
use actix_web::{HttpRequest, HttpResponse, Responder, web};

pub async fn get(state: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    let user_id = match user_id_from_request(&req) {
        Some(id) => id,
        None => return HttpResponse::Unauthorized().finish(),
    };

    match state.users.username_by_id(user_id).await {
        Ok(Some(username)) => HttpResponse::Ok().json(serde_json::json!({ "username": username })),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().body("Something went wrong"),
    }
}
