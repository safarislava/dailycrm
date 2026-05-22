use crate::model::projects::Projects;
use crate::model::stages::Stages;
use crate::model::users::Users;

#[derive(Clone)]
pub struct AppState {
    pub users: Users,
    pub projects: Projects,
    pub stages: Stages,
}
