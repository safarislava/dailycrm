use crate::auth::user_id_from_request;
use crate::state::AppState;
use actix_web::{HttpRequest, HttpResponse, Responder, web};
use uuid::Uuid;

pub async fn get(
    state: web::Data<AppState>,
    request: HttpRequest,
    path: web::Path<(Uuid, i32)>,
) -> impl Responder {
    if user_id_from_request(&request).is_none() {
        return HttpResponse::Unauthorized().finish();
    }
    let (project_id, stage_position) = path.into_inner();
    match state.attachments.list(project_id, stage_position).await {
        Ok(attachments) => HttpResponse::Ok().json(attachments),
        Err(_) => HttpResponse::InternalServerError().body("Something went wrong"),
    }
}