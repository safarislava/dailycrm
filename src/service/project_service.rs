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

    pub async fn get_all_projects(&self) -> Result<Vec<Project>, sqlx::Error> {
        let rows = self.repo.find_all().await?;
        let projects = rows
            .into_iter()
            .map(|(id, title)| Project::new(id, title))
            .collect();
        Ok(projects)
    }

    pub async fn create_project(&self, title: String) -> Result<(), sqlx::Error> {
        self.repo.create(&title).await
    }

    pub async fn delete_project(&self, id: Uuid) -> Result<(), sqlx::Error> {
        self.repo.delete(id).await
    }
}
