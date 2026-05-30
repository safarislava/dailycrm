mod auth;
mod common;
mod cors;
mod db;
mod endpoint;
mod model;
mod routes;
mod state;
mod storage;

use crate::state::AppState;
use crate::storage::Storage;
use actix_web::{App, HttpServer, web};
use std::env;
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    let allowed_origin =
        env::var("ALLOWED_ORIGIN").unwrap_or_else(|_| "http://localhost:5173".to_string());
    let state = web::Data::new(AppState {
        pool: Arc::new(db::connect().await),
        storage: Arc::new(Storage::from_env().await),
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
