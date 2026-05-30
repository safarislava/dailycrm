use crate::model::credential::contract::contentable::Contentable;
use crate::model::project::contract::list::List;
use crate::model::project::deadlines::Deadlines;
use crate::state::AppState;
use actix_web::{HttpResponse, Responder, web};
use futures_util::future::try_join_all;

pub async fn get(state: web::Data<AppState>) -> impl Responder {
    let summaries = match Deadlines::new(state.pool.clone())
        .items()
        .await
    {
        Ok(summaries) => summaries,
        Err(_) => return HttpResponse::InternalServerError().body("Something went wrong"),
    };
    let futures = summaries
        .into_iter()
        .map(|summary| async move { summary.content().await });
    match try_join_all(futures).await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(_) => HttpResponse::InternalServerError().body("Something went wrong"),
    }
}
