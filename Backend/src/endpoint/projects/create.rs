use crate::model::task::contract::task::Task;
use crate::model::task::project::project_registration::ProjectRegistration;
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
    let task = ProjectRegistration::new(state.pool.clone(), body.title.clone());
    match task.done().await {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::InternalServerError().body("Something went wrong"),
    }
}
