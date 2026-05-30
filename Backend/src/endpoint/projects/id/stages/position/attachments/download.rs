use crate::model::project::attachment::Attachment;
use crate::model::task::contract::task::Task;
use crate::model::task::project::attachment_download::AttachmentDownload;
use crate::state::AppState;
use actix_web::{HttpResponse, Responder, web};
use uuid::Uuid;

pub async fn get(state: web::Data<AppState>, path: web::Path<(Uuid, i32, Uuid)>) -> impl Responder {
    let (_, _, attachment_id) = path.into_inner();
    let attachment = Attachment::new(attachment_id);
    let task = AttachmentDownload::new(state.pool.clone(), state.storage.clone(), attachment);
    match task.done().await {
        Ok((stream, content_length, content_type, content_disposition)) => HttpResponse::Ok()
            .content_type(content_type)
            .insert_header(("Content-Disposition", content_disposition))
            .insert_header(("Content-Length", content_length.to_string()))
            .streaming(stream),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}
