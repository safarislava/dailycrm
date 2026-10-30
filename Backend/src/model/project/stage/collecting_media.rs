use crate::model::contract::stage_media::{StageMedia, StageStatus};
use crate::model::project::id::ProjectId;
use crate::model::project::stage::cost::payment::StagePayment;
use crate::model::project::stage::id::StageId;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, sqlx::FromRow, serde::Deserialize, serde::Serialize)]
pub struct StageSummaryItem {
    pub project_id: Uuid,
    pub parent_position: i32,
    pub position: i32,
    pub title: String,
    pub deadline: Option<DateTime<Utc>>,
    pub completed: bool,
    pub gip_confirmed: bool,
    #[sqlx(json)]
    pub payments: Vec<StagePayment>,
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
            self.payments.clone(),
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
        payments: Vec<StagePayment>,
    ) {
        self.items.push(StageSummaryItem {
            project_id: stage_id.project_id().id(),
            parent_position: stage_id.parent_position(),
            position: stage_id.position(),
            title: title.to_string(),
            deadline: status.deadline,
            completed: status.completed,
            gip_confirmed: status.gip_confirmed,
            payments,
            has_act: status.has_act,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::project::id::ProjectId;

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
            vec![
                StagePayment::new("first_advance", Some(100.into()), true),
                StagePayment::new("second_advance", Some(150.into()), false),
                StagePayment::new("final", Some(200.into()), false),
            ],
        );

        let items = media.items();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].project_id, raw_project_id);
        assert_eq!(items[0].parent_position, 0);
        assert_eq!(items[0].position, 1);
        assert_eq!(items[0].title, "Stage 1");
        assert_eq!(items[0].payments.len(), 3);
        assert_eq!(items[0].payments[0].kind, "first_advance");
        assert!(items[0].payments[0].confirmed);
        assert_eq!(items[0].payments[1].kind, "second_advance");
        assert!(!items[0].payments[1].confirmed);
        assert!(items[0].has_act);
    }
}
