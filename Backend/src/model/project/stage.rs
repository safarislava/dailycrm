use crate::model::project::project::Project;

#[derive(Clone)]
pub struct Stage {
    project: Project,
    position: i32,
}

impl Stage {
    pub fn new(project: Project, position: i32) -> Self {
        Stage { project, position }
    }

    pub fn project(&self) -> Project {
        self.project.clone()
    }

    pub fn position(&self) -> i32 {
        self.position
    }
}
