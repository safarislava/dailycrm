use crate::service::project_service::ProjectService;
use crate::service::stage_service::StageService;
use crate::service::user_service::UserService;

#[derive(Clone)]
pub struct AppState {
    pub user_service: UserService,
    pub project_service: ProjectService,
    pub stage_service: StageService,
}
