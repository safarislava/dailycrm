#![allow(clippy::double_must_use)]

use actix_web::{App, HttpServer, web};
use dailycrm::cors;
use dailycrm::db;
use dailycrm::logger::AppLogs;
use dailycrm::logger::RollingLogs;
use dailycrm::mail::Mailer;
use dailycrm::model::cache::memory_cache::MemoryCache;
use dailycrm::model::notification::deadline_digest_notification::DeadlineDigestNotification;
use dailycrm::model::notification::dispatch::NotificationDispatch;
use dailycrm::model::scheduler::contract::schedule::Schedule;
use dailycrm::model::scheduler::cron_event::CronEvent;
use dailycrm::model::scheduler::scheduled_task::ScheduledTask;
use dailycrm::model::scheduler::timetable::Timetable;
use dailycrm::routes;
use dailycrm::state::AppState;
use dailycrm::storage::Storage;
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    let _log_guard = RollingLogs::default_crm().attach();
    tracing::info!("Starting DailyCRM server...");
    let pool = Arc::new(db::pool().await);
    let storage = Arc::new(Storage::from_env().await);
    let mailer = Arc::new(Mailer::from_env());
    let state = web::Data::new(AppState {
        pool: pool.clone(),
        storage: storage.clone(),
        mailer: mailer.clone(),
        project_cache: MemoryCache::new(),
        stage_cache: MemoryCache::new(),
        user_cache: MemoryCache::new(),
    });
    let deadline_schedule = ScheduledTask::new(
        Arc::new(CronEvent::new("0 0 12 * * * *").expect("Valid deadline cron expression")),
        Arc::new(DeadlineDigestNotification::new(
            pool.clone(),
            mailer.clone(),
        )),
    );
    let dispatch_schedule = ScheduledTask::new(
        Arc::new(CronEvent::new("0 * * * * * *").expect("Valid dispatch cron expression")),
        Arc::new(NotificationDispatch::new(pool, mailer)),
    );
    let timetable = Timetable::new(vec![deadline_schedule, dispatch_schedule]);
    actix_web::rt::spawn(async move {
        if let Err(error) = timetable.run().await {
            tracing::error!("schedule stopped: {error}");
        }
    });
    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .wrap(tracing_actix_web::TracingLogger::default())
            .wrap(actix_web::middleware::Compress::default())
            .wrap(cors::rules())
            .wrap(cors::security_headers())
            .configure(routes::configure)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

// TODO Backend tests
