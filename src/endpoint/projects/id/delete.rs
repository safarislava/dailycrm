use actix_web::{HttpResponse, Responder, web};

pub async fn delete(path: web::Path<i64>) -> impl Responder {
    let project_id = path.into_inner();
    HttpResponse::Ok().body(project_id.to_string())
}
