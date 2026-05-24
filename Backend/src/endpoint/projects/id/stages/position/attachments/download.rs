use crate::auth::user_id_from_request;
use crate::state::AppState;
use actix_web::{HttpRequest, HttpResponse, Responder, web};
use uuid::Uuid;

pub async fn get(
    state: web::Data<AppState>,
    request: HttpRequest,
    path: web::Path<(Uuid, i32, Uuid)>,
) -> impl Responder {
    if user_id_from_request(&request).is_none() {
        return HttpResponse::Unauthorized().finish();
    }
    let (project_id, stage_position, attachment_id) = path.into_inner();
    match state.attachments.download(project_id, stage_position, attachment_id).await {
        Ok((data, content_type, content_disposition)) => HttpResponse::Ok()
            .content_type(content_type)
            .insert_header(("Content-Disposition", content_disposition))
            .body(data),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}