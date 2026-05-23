use crate::auth::verify_token;
use crate::state::AppState;
use actix_web::{HttpRequest, HttpResponse, Responder, web};

pub async fn post(state: web::Data<AppState>, request: HttpRequest) -> impl Responder {
    let user_id = match request
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .and_then(|token| verify_token(token).ok())
        .map(|claims| claims.sub)
    {
        Some(id) => id,
        None => return HttpResponse::Unauthorized().finish(),
    };

    match state.invites.create(user_id).await {
        Ok(token) => HttpResponse::Created().json(serde_json::json!({ "token": token })),
        Err(_) => HttpResponse::InternalServerError().body("Something went wrong"),
    }
}