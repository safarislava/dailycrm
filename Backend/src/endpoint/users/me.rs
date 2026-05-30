use crate::auth::UserIdGettable;
use crate::model::credential::contract::contentable::Contentable;
use crate::model::user::detailed_user::DetailedUser;
use crate::model::user::user::User;
use crate::state::AppState;
use actix_web::{HttpRequest, HttpResponse, Responder, web};

pub async fn get(state: web::Data<AppState>, request: HttpRequest) -> impl Responder {
    let user_id = match request.user_id() {
        Some(id) => id,
        None => return HttpResponse::Unauthorized().finish(),
    };
    let user = DetailedUser::new(state.pool.clone(), User::new(user_id));
    match user.username().await {
        Ok(Some(username)) => match username.content().await {
            Ok(c) => HttpResponse::Ok().json(serde_json::json!({ "username": c })),
            Err(_) => HttpResponse::InternalServerError().body("Something went wrong"),
        },
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().body("Something went wrong"),
    }
}
