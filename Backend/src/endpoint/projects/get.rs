use crate::model::credential::contract::contentable::Contentable;
use crate::model::project::contract::list::List;
use crate::model::project::detailed_project::DetailedProject;
use crate::model::project::projects::Projects;
use crate::state::AppState;
use actix_web::{HttpResponse, Responder, web};
use futures_util::future::try_join_all;

pub async fn get(state: web::Data<AppState>) -> impl Responder {
    let projects = match Projects::new(state.pool.clone()).items().await {
        Ok(projects) => projects,
        Err(_) => return HttpResponse::InternalServerError().body("Something went wrong"),
    };
    let futures = projects.into_iter().map(|project| {
        let detailed = DetailedProject::new(state.pool.clone(), project);
        async move { detailed.content().await }
    });
    match try_join_all(futures).await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(_) => HttpResponse::InternalServerError().body("Something went wrong"),
    }
}
