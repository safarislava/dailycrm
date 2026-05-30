use crate::model::credential::contract::contentable::Contentable;
use crate::model::project::contract::list::List;
use crate::model::project::project::Project;
use crate::model::project::stage_summary::StageSummary;
use crate::model::project::stages::Stages;
use crate::state::AppState;
use actix_web::{HttpResponse, Responder, web};
use futures_util::future::try_join_all;
use uuid::Uuid;

pub async fn get(state: web::Data<AppState>, path: web::Path<Uuid>) -> impl Responder {
    let project = Project::new(path.into_inner());
    let stages = match Stages::new(state.pool.clone(), project).items().await {
        Ok(projects) => projects,
        Err(_) => return HttpResponse::InternalServerError().body("Something went wrong"),
    };
    let futures = stages.into_iter().map(|stage| {
        let detailed = StageSummary::new(state.pool.clone(), stage);
        async move { detailed.content().await }
    });
    match try_join_all(futures).await {
        Ok(stages) => HttpResponse::Ok().json(stages),
        Err(_) => HttpResponse::InternalServerError().body("Something went wrong"),
    }
}
