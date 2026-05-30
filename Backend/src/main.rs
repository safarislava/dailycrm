mod auth;
mod common;
mod cors;
mod db;
mod endpoint;
mod mail;
mod model;
mod routes;
mod state;
mod storage;

use crate::mail::Mailer;
use crate::model::schedule::contract::scheduled::Scheduled;
use crate::model::schedule::schedule::Schedule;
use crate::model::schedule::time_of_day::TimeOfDay;
use crate::model::schedule::timetable::Timetable;
use crate::model::task::notification::deadline_digest_notification::DeadlineDigestNotification;
use crate::state::AppState;
use crate::storage::Storage;
use actix_web::{App, HttpServer, web};
use chrono::NaiveTime;
use std::env;
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    let allowed_origin =
        env::var("ALLOWED_ORIGIN").unwrap_or_else(|_| "http://localhost:5173".to_string());
    let pool = Arc::new(db::connect().await);
    let mailer = Arc::new(Mailer::from_env());
    let state = web::Data::new(AppState {
        pool: pool.clone(),
        storage: Arc::new(Storage::from_env().await),
        mailer: mailer.clone(),
    });
    let timetable = Timetable::new(vec![Schedule::new(
        Arc::new(TimeOfDay::new(NaiveTime::from_hms_opt(12, 0, 0).expect("valid time"))),
        Arc::new(DeadlineDigestNotification::new(pool, mailer)),
    )]);
    actix_web::rt::spawn(async move {
        if let Err(error) = timetable.run().await {
            eprintln!("schedule stopped: {error}");
        }
    });
    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .wrap(cors::cors(&allowed_origin))
            .wrap(cors::security_headers())
            .configure(routes::configure)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
