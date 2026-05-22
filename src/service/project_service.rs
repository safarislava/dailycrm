use crate::model::project::Project;
use crate::repository::project_repository::ProjectRepository;
use uuid::Uuid;

#[derive(Clone)]
pub struct ProjectService {
    repo: ProjectRepository,
}

impl ProjectService {
    pub fn new(repo: ProjectRepository) -> Self {
        Self { repo }
    }

    pub async fn projects(&self) -> Result<Vec<Project>, sqlx::Error> {
        let rows = self.repo.projects().await?;
        Ok(rows.into_iter().map(|(id, title)| Project::new(id, title)).collect())
    }

    pub async fn save(&self, title: String) -> Result<(), sqlx::Error> {
        self.repo.save(&title).await
    }

    pub async fn remove(&self, id: Uuid) -> Result<(), sqlx::Error> {
        self.repo.remove(id).await
    }
}
