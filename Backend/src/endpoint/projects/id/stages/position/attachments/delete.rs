use crate::state::AppState;
use actix_web::{HttpResponse, Responder, web};
use uuid::Uuid;

pub async fn delete(
    state: web::Data<AppState>,
    path: web::Path<(Uuid, i32, Uuid)>,
) -> impl Responder {
    let (project_id, _stage_position, attachment_id) = path.into_inner();
    match state.attachments.delete(attachment_id, project_id).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}
