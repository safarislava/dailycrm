use crate::model::contract::stage_media::{StageCosts, StageMedia, StageStatus};
use crate::model::project::id::ProjectId;
use crate::model::project::stage::cost::contract::cost::Cost;
use crate::model::project::stage::id::StageId;
use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Clone, Debug, FromRow, serde::Deserialize, serde::Serialize)]
pub struct StageSummaryItem {
    pub project_id: Uuid,
    pub parent_position: i32,
    pub position: i32,
    pub title: String,
    pub deadline: Option<DateTime<Utc>>,
    pub completed: bool,
    pub gip_confirmed: bool,
    pub advance_cost: Option<Box<dyn Cost>>,
    pub advance_confirmed: bool,
    pub final_cost: Option<Box<dyn Cost>>,
    pub final_confirmed: bool,
    pub has_act: bool,
}

impl StageSummaryItem {
    pub fn print<M: StageMedia>(&self, media: &mut M) {
        let stage_id = StageId::new_substage(
            ProjectId::from(self.project_id),
            self.parent_position,
            self.position,
        );
        media.add_stage(
            stage_id,
            &self.title,
            StageStatus {
                deadline: self.deadline,
                completed: self.completed,
                gip_confirmed: self.gip_confirmed,
                has_act: self.has_act,
            },
            StageCosts {
                advance_cost: self.advance_cost.clone(),
                advance_confirmed: self.advance_confirmed,
                final_cost: self.final_cost.clone(),
                final_confirmed: self.final_confirmed,
            },
        );
    }
}

#[derive(Clone, Debug, Default)]
pub struct CollectingStageMedia {
    items: Vec<StageSummaryItem>,
}

impl CollectingStageMedia {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn items(self) -> Vec<StageSummaryItem> {
        self.items
    }
}

impl StageMedia for CollectingStageMedia {
    fn add_stage(
        &mut self,
        stage_id: StageId,
        title: &str,
        status: StageStatus,
        costs: StageCosts,
    ) {
        self.items.push(StageSummaryItem {
            project_id: stage_id.project_id().id(),
            parent_position: stage_id.parent_position(),
            position: stage_id.position(),
            title: title.to_string(),
            deadline: status.deadline,
            completed: status.completed,
            gip_confirmed: status.gip_confirmed,
            advance_cost: costs.advance_cost,
            advance_confirmed: costs.advance_confirmed,
            final_cost: costs.final_cost,
            final_confirmed: costs.final_confirmed,
            has_act: status.has_act,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::project::id::ProjectId;
    use rust_decimal::Decimal;

    #[test]
    fn collects_added_stages_and_substages() {
        let mut media = CollectingStageMedia::new();
        let raw_project_id = Uuid::new_v4();
        let project_id = ProjectId::from(raw_project_id);
        let stage_id = StageId::new_substage(project_id, 0, 1);

        media.add_stage(
            stage_id,
            "Stage 1",
            StageStatus {
                deadline: None,
                completed: false,
                gip_confirmed: true,
                has_act: true,
            },
            StageCosts {
                advance_cost: Some(100.into()),
                advance_confirmed: true,
                final_cost: Some(200.into()),
                final_confirmed: false,
            },
        );

        let items = media.items();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].project_id, raw_project_id);
        assert_eq!(items[0].parent_position, 0);
        assert_eq!(items[0].position, 1);
        assert_eq!(items[0].title, "Stage 1");
        assert_eq!(
            items[0].advance_cost.as_ref().map(|c| c.as_decimal()),
            Some(Ok(Decimal::from(100)))
        );
        assert_eq!(items[0].has_act, true);
    }
}
