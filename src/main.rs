mod endpoint;
mod user;

use std::env;
use actix_cors::Cors;
use actix_web::{App, HttpServer};
use sqlx::postgres::PgPoolOptions;
use crate::endpoint::approx_service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("Environment variable DATABASE_URL does not exist");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to migrate the database");

    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())
            .service(approx_service)
    })
        .bind(("localhost", 8080))?
        .run()
        .await
}