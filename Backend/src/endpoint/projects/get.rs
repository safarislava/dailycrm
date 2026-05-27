use crate::state::AppState;
use actix_web::{HttpResponse, Responder, web};
use futures_util::future::try_join_all;

pub async fn get(state: web::Data<AppState>) -> impl Responder {
    let projects = match state.projects.list().await {
        Ok(p) => p,
        Err(_) => return HttpResponse::InternalServerError().body("Something went wrong"),
    };
    let futures: Vec<_> = projects.iter().map(|p| p.data()).collect();
    match try_join_all(futures).await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(_) => HttpResponse::InternalServerError().body("Something went wrong"),
    }
}