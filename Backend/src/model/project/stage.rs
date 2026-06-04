use crate::model::project::project::Project;

#[derive(Clone)]
pub struct Stage {
    project: Project,
    parent_position: i32,
    position: i32,
}

impl Stage {
    pub fn new(project: Project, position: i32) -> Self {
        Stage { project, parent_position: 0, position }
    }

    pub fn new_substage(project: Project, parent_position: i32, position: i32) -> Self {
        Stage { project, parent_position, position }
    }

    pub fn project(&self) -> Project {
        self.project.clone()
    }

    pub fn parent_position(&self) -> i32 {
        self.parent_position
    }

    pub fn position(&self) -> i32 {
        self.position
    }
}