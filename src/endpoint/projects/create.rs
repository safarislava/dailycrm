use crate::state::AppState;
use actix_web::{HttpResponse, Responder, web};

#[derive(serde::Deserialize)]
pub struct CreateProjectDto {
    title: String,
}

pub async fn create(
    state: web::Data<AppState>,
    body: web::Json<CreateProjectDto>,
) -> impl Responder {
    match state
        .projects
        .register(&body.title)
        .await
    {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::InternalServerError().body("Something went wrong"),
    }
}
