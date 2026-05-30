use crate::storage::Storage;
use sqlx::PgPool;
use std::sync::Arc;

pub struct AppState {
    pub pool: Arc<PgPool>,
    pub storage: Arc<Storage>,
}
