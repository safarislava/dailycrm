mod endpoint;
mod model;
mod state;

use crate::model::projects::Projects;
use crate::model::stages::Stages;
use crate::model::users::Users;
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

    let state = web::Data::new(AppState {
        users: Users::new(pool.clone()),
        projects: Projects::new(pool.clone()),
        stages: Stages::new(pool.clone()),
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
                web::resource("/deadlines")
                    .get(endpoint::deadlines::get::get),
            )
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
                        web::resource("/{project_id}/title")
                            .patch(endpoint::projects::id::rename::patch),
                    )
                    .service(
                        web::resource("/{project_id}/stages")
                            .get(endpoint::projects::id::stages::get::get)
                            .post(endpoint::projects::id::stages::create::create),
                    )
                    .service(
                        web::resource("/{project_id}/stages/{stage_id}")
                            .post(endpoint::projects::id::stages::position::create::create)
                            .get(endpoint::projects::id::stages::position::get::get)
                            .delete(endpoint::projects::id::stages::position::delete::delete),
                    )
                    .service(
                        web::resource("/{project_id}/stages/{stage_id}/title")
                            .patch(endpoint::projects::id::stages::position::title::patch),
                    )
                    .service(
                        web::resource("/{project_id}/stages/{stage_id}/deadline")
                            .patch(endpoint::projects::id::stages::position::deadline::patch),
                    )
                    .service(
                        web::resource("/{project_id}/stages/{stage_id}/description")
                            .patch(endpoint::projects::id::stages::position::description::patch),
                    )
                    .service(
                        web::resource("/{project_id}/stages/{stage_id}/cost")
                            .patch(endpoint::projects::id::stages::position::cost::patch),
                    )
                    .service(
                        web::resource("/{project_id}/stages/{stage_id}/completed")
                            .patch(endpoint::projects::id::stages::position::completed::patch),
                    ),
            )
            .service(
                web::scope("/users")
                    .service(web::resource("").post(endpoint::users::create::create)),
            ),
    );
}
