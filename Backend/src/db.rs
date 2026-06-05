use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::env;

pub async fn connect() -> PgPool {
    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&url)
        .await
        .expect("Failed to connect to database");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to migrate the database");
    pool
}
