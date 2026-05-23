use crate::state::AppState;
use actix_web::{HttpResponse, Responder, web};

#[derive(serde::Deserialize)]
pub struct CreateUserDto {
    username: String,
    password: String,
}

pub async fn create(state: web::Data<AppState>, body: web::Json<CreateUserDto>) -> impl Responder {
    let password = body.password.clone();
    let password_hash = match actix_web::rt::task::spawn_blocking(move || {
        bcrypt::hash(password, bcrypt::DEFAULT_COST)
    })
    .await
    {
        Ok(Ok(hash)) => hash,
        _ => return HttpResponse::InternalServerError().body("Something went wrong"),
    };

    match state.users.register(&body.username, &password_hash).await {
        Ok(_) => HttpResponse::Created().finish(),
        Err(sqlx::Error::Database(_)) => HttpResponse::Conflict().body("User already exists"),
        Err(_) => HttpResponse::InternalServerError().body("Something went wrong"),
    }
}