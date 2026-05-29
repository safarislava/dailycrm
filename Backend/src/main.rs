mod auth;
mod common;
mod cors;
mod db;
mod endpoint;
mod model;
mod routes;
mod state;
mod storage;

use crate::model::project::deadlines::PgDeadlines;
use crate::model::project::projects::PgProjects;
use crate::model::session::refresh_tokens::PgRefreshTokens;
use crate::model::user::users::PgUsers;
use crate::state::AppState;
use actix_web::{App, HttpServer, web};
use std::env;
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    let allowed_origin =
        env::var("ALLOWED_ORIGIN").unwrap_or_else(|_| "http://localhost:5173".to_string());

    let pool = db::connect().await;
    let storage = storage::Storage::from_env().await;

    let state = web::Data::new(AppState {
        pool: pool.clone(),
        users: Arc::new(PgUsers::new(pool.clone())),
        projects: Arc::new(PgProjects::new(pool.clone(), storage)),
        refresh_tokens: Arc::new(PgRefreshTokens::new(pool.clone())),
        deadlines: Arc::new(PgDeadlines::new(pool)),
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
