use crate::model::contract::box_error::BoxError;
use crate::model::contract::comment_media::CommentMedia;
use crate::model::contract::printer::Printer;
use crate::model::project::stage::id::StageId;
use crate::storage::Storage;
use chrono::{DateTime, Utc};
use std::sync::Arc;
use uuid::Uuid;

pub struct ArchivedCommentSummaries {
    storage: Arc<Storage>,
    stage_id: StageId,
    before: Option<Uuid>,
}

impl ArchivedCommentSummaries {
    pub fn new(storage: Arc<Storage>, stage_id: StageId, before: Option<Uuid>) -> Self {
        Self {
            storage,
            stage_id,
            before,
        }
    }
}

#[async_trait::async_trait]
impl<M: CommentMedia> Printer<M> for ArchivedCommentSummaries {
    async fn print(&self, media: &mut M) -> Result<(), BoxError> {
        let storage_key = format!("archives/{}.json", self.stage_id.project_id().id());
        let bytes = self.storage.read_bytes(&storage_key).await?;
        let payload: serde_json::Value = serde_json::from_slice(&bytes)?;

        if let Some(comments) = payload["comments"].as_array() {
            let mut matching: Vec<&serde_json::Value> = comments
                .iter()
                .filter(|c| {
                    let parent = c["parent_position"].as_i64().unwrap_or(0) as i32;
                    let pos = c["stage_position"].as_i64().unwrap_or(0) as i32;
                    parent == self.stage_id.parent_position() && pos == self.stage_id.position()
                })
                .collect();

            if let Some(before_id) = self.before
                && let Some(pivot_idx) = matching.iter().position(|c| {
                    c["id"]
                        .as_str()
                        .and_then(|s| Uuid::parse_str(s).ok())
                        .map(|id| id == before_id)
                        .unwrap_or(false)
                })
            {
                matching.truncate(pivot_idx);
            }

            for c in matching {
                let id = c["id"]
                    .as_str()
                    .and_then(|s| Uuid::parse_str(s).ok())
                    .unwrap_or_else(Uuid::nil);
                let text = c["text"].as_str().unwrap_or("");
                let author = c["author"].as_str().unwrap_or("");
                let is_system = c["is_system"].as_bool().unwrap_or(false);
                let created_at = c["created_at"]
                    .as_str()
                    .and_then(|t| t.parse::<DateTime<Utc>>().ok())
                    .unwrap_or_else(Utc::now);
                let is_pinned = c["is_pinned"].as_bool().unwrap_or(false);

                media.add_comment(id, text, author, is_system, created_at, is_pinned);
            }
        }
        Ok(())
    }
}
