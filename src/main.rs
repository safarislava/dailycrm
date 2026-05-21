mod endpoint;
mod models;

use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use sqlx::postgres::PgPoolOptions;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    let database_url =
        env::var("DATABASE_URL").expect("Environment variable DATABASE_URL does not exist");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to migrate the database");

    HttpServer::new(|| App::new().wrap(Cors::permissive()).configure(configure_api))
        .bind(("localhost", 8080))?
        .run()
        .await
}

fn configure_api(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/api")
            .service(
                web::scope("/projects")
                    .service(
                        web::resource("")
                            .get(endpoint::projects::get::get)
                            .post(endpoint::projects::create::create),
                    )
                    .service(
                        web::resource("/{project_id}")
                            .delete(endpoint::projects::id::delete::delete),
                    )
                    .service(
                        web::resource("/{project_id}/stages")
                            .get(endpoint::projects::id::stages::get::get)
                            .post(endpoint::projects::id::stages::create::create),
                    )
                    .service(
                        web::resource("/{project_id}/stages/{stage_id}")
                            .get(endpoint::projects::id::stages::id::get::get)
                            .delete(endpoint::projects::id::stages::id::delete::delete),
                    ),
            )
            .service(
                web::scope("/users")
                    .service(
                        web::resource("")
                            .get(endpoint::users::get::get)),
            ),
    );
}
