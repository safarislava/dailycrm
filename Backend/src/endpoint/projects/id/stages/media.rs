use crate::model::contract::stage_media::{StageCosts, StageMedia, StageStatus};
use crate::model::project::stage::cost::valid_cost::ValidCost;
use crate::model::project::stage::id::StageId;
use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
struct JsonStageItem {
    project_id: Uuid,
    parent_position: i32,
    position: i32,
    title: String,
    deadline: Option<DateTime<Utc>>,
    completed: bool,
    gip_confirmed: bool,
    advance_cost: Option<ValidCost>,
    advance_confirmed: bool,
    final_cost: Option<ValidCost>,
    final_confirmed: bool,
    has_act: bool,
}

#[derive(Default, Serialize)]
#[serde(transparent)]
pub struct JsonStageMedia {
    items: Vec<JsonStageItem>,
}

impl StageMedia for JsonStageMedia {
    fn add_stage(
        &mut self,
        stage_id: StageId,
        title: &str,
        status: StageStatus,
        costs: StageCosts,
    ) {
        self.items.push(JsonStageItem {
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

    #[test]
    fn serializes_stage_media_to_json() {
        let mut media = JsonStageMedia::default();
        let project_id = ProjectId::from(Uuid::nil());
        let stage_id = StageId::new_substage(project_id, 0, 1);

        media.add_stage(
            stage_id,
            "Concept Design",
            StageStatus {
                deadline: None,
                completed: false,
                gip_confirmed: true,
                has_act: true,
            },
            StageCosts {
                advance_cost: Some(50000.into()),
                advance_confirmed: true,
                final_cost: Some(100000.into()),
                final_confirmed: false,
            },
        );

        let json = serde_json::to_string(&media).unwrap();
        assert!(json.contains("\"title\":\"Concept Design\""));
        assert!(
            json.contains("\"advance_cost\":50000") || json.contains("\"advance_cost\":\"50000\"")
        );
        assert!(json.contains("\"has_act\":true"));
    }
}
