use std::sync::Arc;
use crate::project::Project;

pub struct Projects {
    projects: Arc<[Project]>,
}