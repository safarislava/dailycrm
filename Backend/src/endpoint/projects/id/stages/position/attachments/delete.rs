use crate::model::project::attachment::Attachment;
use crate::model::task::contract::task::Task;
use crate::model::task::project::attachment_removal::AttachmentRemoval;
use crate::state::AppState;
use actix_web::{HttpResponse, Responder, web};
use uuid::Uuid;

pub async fn delete(
    state: web::Data<AppState>,
    path: web::Path<(Uuid, i32, Uuid)>,
) -> impl Responder {
    let (_, _, attachment_id) = path.into_inner();
    let attachment = Attachment::new(attachment_id);
    let task = AttachmentRemoval::new(state.pool.clone(), state.storage.clone(), attachment);
    match task.done().await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}
