use crate::model::project::Project;
use std::sync::Arc;

pub struct Projects {
    projects: Arc<[Project]>,
}
