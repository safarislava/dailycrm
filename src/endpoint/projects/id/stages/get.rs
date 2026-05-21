use actix_web::{HttpResponse, Responder, web};

pub async fn get(path: web::Path<(u64, u64)>) -> impl Responder {
    let (project_id, stage_id) = path.into_inner();
    HttpResponse::Ok()
}
