use crate::state::AppState;
use actix_web::{HttpResponse, Responder, web};

#[derive(serde::Deserialize)]
pub struct CreateUserDto {
    username: String,
    password_hash: String,
}

pub async fn create(state: web::Data<AppState>, body: web::Json<CreateUserDto>) -> impl Responder {
    match state
        .users
        .register(&body.username, &body.password_hash)
        .await
    {
        Ok(_) => HttpResponse::Created().finish(),
        Err(sqlx::Error::Database(_)) => HttpResponse::Conflict().body("User already exists"),
        Err(_) => HttpResponse::InternalServerError().body("Something went wrong"),
    }
}
