use crate::model::project::contract::deadlines::Deadlines;
use crate::model::project::contract::projects::Projects;
use crate::model::project::contract::users::Users;
use std::sync::Arc;

pub struct AppState {
    pub pool: sqlx::PgPool,
    pub users: Arc<dyn Users>,
    pub projects: Arc<dyn Projects>,
    pub deadlines: Arc<dyn Deadlines>,
}
