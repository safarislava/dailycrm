use crate::model::contract::stage_media::{StageMedia, StageStatus};
use crate::model::project::stage::cost::payment::StagePayment;
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
    payments: Vec<StagePayment>,
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
        payments: Vec<StagePayment>,
    ) {
        self.items.push(JsonStageItem {
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
            vec![
                StagePayment::new("first_advance", Some(50000.into()), true),
                StagePayment::new("second_advance", Some(75000.into()), false),
                StagePayment::new("final", Some(100000.into()), false),
            ],
        );

        let json = serde_json::to_string(&media).unwrap();
        assert!(json.contains("\"title\":\"Concept Design\""));
        assert!(
            json.contains(
                r#"{"kind":"first_advance","cost":50000,"confirmed":true,"tranches":[]}"#
            ) || json.contains(
                r#"{"kind":"first_advance","cost":"50000","confirmed":true,"tranches":[]}"#
            )
        );
        assert!(
            json.contains(
                r#"{"kind":"second_advance","cost":75000,"confirmed":false,"tranches":[]}"#
            ) || json.contains(
                r#"{"kind":"second_advance","cost":"75000","confirmed":false,"tranches":[]}"#
            )
        );
        assert!(json.contains("\"has_act\":true"));
    }
}
