mod endpoint;
mod model;
mod repository;
mod service;
mod state;

use crate::repository::project_repository::ProjectRepository;
use crate::repository::stage_repository::StageRepository;
use crate::repository::user_repository::UserRepository;
use crate::service::project_service::ProjectService;
use crate::service::stage_service::StageService;
use crate::service::user_service::UserService;
use crate::state::AppState;
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

    let user_repo = UserRepository::new(pool.clone());
    let project_repo = ProjectRepository::new(pool.clone());
    let stage_repo = StageRepository::new(pool.clone());

    let user_service = UserService::new(user_repo);
    let project_service = ProjectService::new(project_repo);
    let stage_service = StageService::new(stage_repo);

    let state = web::Data::new(AppState {
        user_service,
        project_service,
        stage_service,
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .wrap(Cors::permissive())
            .configure(configure_api)
    })
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
                    .service(web::resource("").post(endpoint::users::create::create)),
            ),
    );
}
