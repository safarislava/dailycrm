use crate::model::contract::box_error::BoxError;
use crate::model::contract::printer::Printer;
use crate::model::contract::stage_media::{StageMedia, StageStatus};
use crate::model::project::id::ProjectId;
use crate::model::project::stage::cost::payment::StagePayment;
use crate::model::project::stage::id::StageId;
use crate::storage::Storage;
use chrono::{DateTime, Utc};
use std::sync::Arc;

pub struct ArchivedStageSummaries {
    storage: Arc<Storage>,
    project_id: ProjectId,
}

impl ArchivedStageSummaries {
    pub fn new(storage: Arc<Storage>, project_id: ProjectId) -> Self {
        Self {
            storage,
            project_id,
        }
    }
}

#[async_trait::async_trait]
impl<M: StageMedia> Printer<M> for ArchivedStageSummaries {
    async fn print(&self, media: &mut M) -> Result<(), BoxError> {
        let storage_key = format!("archives/{}.json", self.project_id.id());
        let bytes = self.storage.read_bytes(&storage_key).await?;
        let payload: serde_json::Value = serde_json::from_slice(&bytes)?;

        if let Some(stages) = payload["stages"].as_array() {
            for s in stages {
                let parent_position = s["parent_position"].as_i64().unwrap_or(0) as i32;
                let position = s["position"].as_i64().unwrap_or(0) as i32;
                let title = s["title"].as_str().unwrap_or("");
                let deadline = s["deadline"]
                    .as_str()
                    .and_then(|d| d.parse::<DateTime<Utc>>().ok());
                let completed = s["completed"].as_bool().unwrap_or(false);
                let gip_confirmed = s["gip_confirmed"].as_bool().unwrap_or(false);
                let has_act = s["has_act"].as_bool().unwrap_or(false);

                let payments: Vec<StagePayment> =
                    serde_json::from_value(s["payments"].clone()).unwrap_or_default();

                let stage_id = StageId::new_substage(self.project_id, parent_position, position);
                media.add_stage(
                    stage_id,
                    title,
                    StageStatus {
                        deadline,
                        completed,
                        gip_confirmed,
                        has_act,
                    },
                    payments,
                );
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::project::stage::collecting_media::CollectingStageMedia;
    use opendal::Operator;
    use opendal::services::Memory;
    use serde_json::json;
    use uuid::Uuid;

    #[actix_web::test]
    async fn prints_stages_from_archive_json() {
        let op = Operator::new(Memory::default()).unwrap();
        let storage = Arc::new(Storage::new(op));
        let project_id = ProjectId::new(Uuid::new_v4());

        let payload = json!({
            "id": project_id.id(),
            "title": "Archived Project",
            "stages": [
                {
                    "parent_position": 0,
                    "position": 1,
                    "title": "Stage 1",
                    "deadline": "2026-12-31T00:00:00Z",
                    "completed": true,
                    "gip_confirmed": true,
                    "has_act": true,
                    "payments": [
                        {
                            "kind": "first_advance",
                            "cost": "100.50",
                            "confirmed": true,
                            "tranches": []
                        }
                    ]
                }
            ]
        });

        let key = format!("archives/{}.json", project_id.id());
        storage
            .write_bytes(
                &key,
                serde_json::to_vec(&payload).unwrap(),
                "application/json",
            )
            .await
            .unwrap();

        let summaries = ArchivedStageSummaries::new(storage, project_id);
        let mut media = CollectingStageMedia::new();
        summaries.print(&mut media).await.unwrap();

        let items = media.items();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].title, "Stage 1");
        assert_eq!(items[0].position, 1);
        assert!(items[0].completed);
        assert!(items[0].gip_confirmed);
        assert!(items[0].has_act);
        assert_eq!(items[0].payments.len(), 1);
        assert_eq!(items[0].payments[0].kind, "first_advance");
    }
}
