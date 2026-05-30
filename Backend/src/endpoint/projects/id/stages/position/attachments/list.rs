use crate::model::credential::contract::contentable::Contentable;
use crate::model::project::attachments::Attachments;
use crate::model::project::contract::list::List;
use crate::model::project::detailed_attachment::DetailedAttachment;
use crate::model::project::project::Project;
use crate::model::project::stage::Stage;
use crate::state::AppState;
use actix_web::{HttpResponse, Responder, web};
use futures_util::future::try_join_all;
use uuid::Uuid;

pub async fn get(state: web::Data<AppState>, path: web::Path<(Uuid, i32)>) -> impl Responder {
    let (project_id, stage_position) = path.into_inner();
    let project = Project::new(project_id);
    let stage = Stage::new(project, stage_position);
    let attachments = Attachments::new(state.pool.clone(), stage);
    let list = match attachments.items().await {
        Ok(l) => l,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };
    let futures = list.into_iter().map(|attachment| {
        let detailed = DetailedAttachment::new(state.pool.clone(), attachment);
        async move { detailed.content().await }
    });
    match try_join_all(futures).await {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(_) => HttpResponse::InternalServerError().body("Something went wrong"),
    }
}
