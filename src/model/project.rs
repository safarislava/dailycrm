use crate::model::stages::Stages;
use uuid::Uuid;

#[derive(serde::Serialize)]
pub struct Project {
    id: Uuid,
    title: String,
}

pub struct StagedProject {
    project: Project,
    stages: Stages,
}

impl Project {
    pub fn new(id: Uuid, title: String) -> Self {
        Project { id, title }
    }
}

impl StagedProject {
    pub fn new(project: Project, stages: Stages) -> Self {
        StagedProject { project, stages }
    }
}
