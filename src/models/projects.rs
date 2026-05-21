use std::sync::Arc;
use crate::models::project::Project;

pub struct Projects {
    projects: Arc<[Project]>,
}