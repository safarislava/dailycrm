use crate::model::stage::{DetailedStage, Stage};
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

    pub async fn stages(&self, project_id: Uuid) -> Result<Vec<Stage>, sqlx::Error> {
        let rows = self.repo.stages(project_id).await?;
        Ok(rows
            .into_iter()
            .map(|row| Stage::new(row.project_id, row.id, row.title))
            .collect())
    }

    pub async fn save(
        &self,
        project_id: Uuid,
        position: i64,
        title: String,
    ) -> Result<(), sqlx::Error> {
        self.repo.save(project_id, position, title).await
    }

    pub async fn detailed_stage(
        &self,
        project_id: Uuid,
        stage_id: Uuid,
    ) -> Result<DetailedStage, sqlx::Error> {
        let row = self.repo.stage(project_id, stage_id).await?;
        let base = Stage::new(row.project_id, row.id, row.title.clone());
        Ok(DetailedStage::new(base, row.description, row.deadline, row.cost))
    }

    pub async fn remove(&self, project_id: Uuid, stage_id: Uuid) -> Result<(), sqlx::Error> {
        self.repo.remove(project_id, stage_id).await
    }
}
