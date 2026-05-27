use crate::state::AppState;
use actix_web::{HttpResponse, Responder, web};
use uuid::Uuid;

pub async fn get(state: web::Data<AppState>, path: web::Path<(Uuid, i32, Uuid)>) -> impl Responder {
    let (project_id, stage_position, attachment_id) = path.into_inner();

    let attachment = match state
        .projects
        .project_link(project_id)
        .stages()
        .stage_link(stage_position)
        .attachments()
        .attachment_by_id(attachment_id)
        .await
    {
        Ok(a) => a,
        Err(_) => return HttpResponse::NotFound().finish(),
    };

    match attachment.content(&state.storage).await {
        Ok((data, content_type, content_disposition)) => HttpResponse::Ok()
            .content_type(content_type)
            .insert_header(("Content-Disposition", content_disposition))
            .body(data),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
