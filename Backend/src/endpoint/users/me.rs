use crate::endpoint::auth_header::UserHeader;
use crate::model::credential::contract::contentable::Contentable;
use crate::model::user::detailed_user::DetailedUser;
use crate::state::AppState;
use actix_web::{HttpRequest, HttpResponse, Responder, web};

pub async fn get(state: web::Data<AppState>, request: HttpRequest) -> impl Responder {
    let user = match request.user() {
        Some(u) => u,
        None => return HttpResponse::Unauthorized().finish(),
    };
    let user = DetailedUser::new(state.pool.clone(), user);
    let (username, email, notifications_enabled) = match (
        user.username().await,
        user.email().await,
        user.notifications_enabled().await,
    ) {
        (Ok(u), Ok(e), Ok(n)) => (u, e, n),
        _ => return HttpResponse::InternalServerError().body("Something went wrong"),
    };
    match (username, email, notifications_enabled) {
        (Some(username), Some(email), Some(notifications_enabled)) => {
            let username = match username.content().await {
                Ok(u) => u,
                Err(_) => return HttpResponse::InternalServerError().body("Something went wrong"),
            };
            HttpResponse::Ok().json(serde_json::json!({
                "username": username,
                "email": email,
                "notifications_enabled": notifications_enabled,
            }))
        }
        _ => HttpResponse::NotFound().finish(),
    }
}
