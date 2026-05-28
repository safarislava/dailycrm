mod auth;
mod contract;
mod endpoint;
mod model;
mod state;
mod storage;

use crate::auth::JwtMiddleware;
use crate::model::deadlines::PgDeadlines;
use crate::model::invites::PgInvites;
use crate::model::projects::PgProjects;
use crate::model::refresh_tokens::PgRefreshTokens;
use crate::model::users::PgUsers;
use crate::state::AppState;
use crate::storage::Storage;
use std::sync::Arc;

use aws_sdk_s3::Client;
use aws_sdk_s3::config::{BehaviorVersion, Builder, Credentials, Region};

use actix_cors::Cors;
use actix_governor::governor::clock::QuantaInstant;
use actix_governor::governor::middleware::NoOpMiddleware;
use actix_governor::{Governor, GovernorConfig, GovernorConfigBuilder, PeerIpKeyExtractor};
use actix_web::middleware::DefaultHeaders;
use actix_web::{App, HttpServer, http, web};
use sqlx::postgres::PgPoolOptions;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    let database_url =
        env::var("DATABASE_URL").expect("Environment variable DATABASE_URL does not exist");

    let allowed_origin =
        env::var("ALLOWED_ORIGIN").unwrap_or_else(|_| "http://localhost:5173".to_string());

    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to migrate the database");

    let minio_endpoint = env::var("MINIO_ENDPOINT").expect("MINIO_ENDPOINT must be set");
    let minio_access_key = env::var("MINIO_ACCESS_KEY").expect("MINIO_ACCESS_KEY must be set");
    let minio_secret_key = env::var("MINIO_SECRET_KEY").expect("MINIO_SECRET_KEY must be set");

    let s3_credentials =
        Credentials::new(&minio_access_key, &minio_secret_key, None, None, "minio");
    let s3_client = Client::from_conf(
        Builder::new()
            .behavior_version(BehaviorVersion::latest())
            .endpoint_url(&minio_endpoint)
            .credentials_provider(s3_credentials)
            .region(Region::new("us-east-1"))
            .force_path_style(true)
            .build(),
    );

    let storage = Storage::new(s3_client);
    storage.ensure_bucket().await;

    let state = web::Data::new(AppState {
        users: Arc::new(PgUsers::new(pool.clone())),
        projects: Arc::new(PgProjects::new(pool.clone(), storage.clone())),
        invites: Arc::new(PgInvites::new(pool.clone())),
        refresh_tokens: Arc::new(PgRefreshTokens::new(pool.clone())),
        deadlines: Arc::new(PgDeadlines::new(pool.clone())),
    });

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&allowed_origin)
            .allowed_methods(vec!["GET", "POST", "PUT", "PATCH", "DELETE", "OPTIONS"])
            .allowed_headers(vec![
                http::header::AUTHORIZATION,
                http::header::CONTENT_TYPE,
            ])
            .supports_credentials()
            .max_age(3600);

        let security_headers = DefaultHeaders::new()
            .add(("X-Content-Type-Options", "nosniff"))
            .add(("X-Frame-Options", "DENY"))
            .add(("Referrer-Policy", "strict-origin-when-cross-origin"));

        App::new()
            .app_data(state.clone())
            .wrap(cors)
            .wrap(security_headers)
            .configure(configure_api)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

fn login_governor() -> GovernorConfig<PeerIpKeyExtractor, NoOpMiddleware<QuantaInstant>> {
    GovernorConfigBuilder::default()
        .seconds_per_request(1)
        .burst_size(5)
        .finish()
        .unwrap()
}

fn configure_api(config: &mut web::ServiceConfig) {
    let governor = login_governor();
    config.service(
        web::scope("/api")
            .service(
                web::resource("/auth/login")
                    .wrap(Governor::new(&governor))
                    .post(endpoint::auth::login::post),
            )
            .service(web::resource("/auth/refresh").post(endpoint::auth::refresh::post))
            .service(web::resource("/auth/logout").post(endpoint::auth::logout::post))
            .service(web::resource("/users").post(endpoint::users::create::create))
            .service(
                web::scope("/users/me")
                    .wrap(JwtMiddleware)
                    .service(web::resource("").get(endpoint::users::me::get))
                    .service(web::resource("/username").patch(endpoint::users::username::patch))
                    .service(web::resource("/password").patch(endpoint::users::password::patch)),
            )
            .service(
                web::scope("")
                    .wrap(JwtMiddleware)
                    .service(web::resource("/invites").post(endpoint::invites::create::post))
                    .service(
                        web::scope("/projects")
                            .service(web::resource("/deadlines").get(endpoint::projects::deadlines::get::get))
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
                            )
                            .service(
                                web::resource("/{project_id}/stages/{stage_id}/attachments")
                                    .get(endpoint::projects::id::stages::position::attachments::list::get)
                                    .post(endpoint::projects::id::stages::position::attachments::upload::post),
                            )
                            .service(
                                web::resource("/{project_id}/stages/{stage_id}/attachments/{attachment_id}/download")
                                    .get(endpoint::projects::id::stages::position::attachments::download::get),
                            )
                            .service(
                                web::resource("/{project_id}/stages/{stage_id}/attachments/{attachment_id}")
                                    .delete(endpoint::projects::id::stages::position::attachments::delete::delete),
                            ),
                    ),
            ),
    );
}
