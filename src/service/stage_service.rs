use crate::model::stage::Stage;
use crate::repository::stage_repository::StageRepository;
use uuid::Uuid;

#[derive(Clone)]
pub struct StageService {
    repo: StageRepository,
}

impl StageService {
    pub fn new(repo: StageRepository) -> Self {
        Self { repo }
    }

    pub async fn get_stages_for_project(
        &self,
        project_id: Uuid,
    ) -> Result<Vec<Stage>, sqlx::Error> {
        let rows = self.repo.find_by_project_id(project_id).await?;
        let stages = rows
            .into_iter()
            .map(|row| Stage::new_from_row(row))
            .collect();
        Ok(stages)
    }

    pub async fn create_stage(
        &self,
        project_id: Uuid,
        position: i64,
        title: String,
    ) -> Result<(), sqlx::Error> {
        self.repo.create(project_id, position, title).await
    }

    pub async fn get_stage_by_id(
        &self,
        project_id: Uuid,
        stage_id: Uuid,
    ) -> Result<Stage, sqlx::Error> {
        let row = self.repo.find_by_id(project_id, stage_id).await?;
        let stage = Stage::new_from_row(row);
        Ok(stage)
    }

    pub async fn delete_stage(&self, project_id: Uuid, stage_id: Uuid) -> Result<(), sqlx::Error> {
        self.repo.delete(project_id, stage_id).await
    }
}
