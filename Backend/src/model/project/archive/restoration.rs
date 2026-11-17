use crate::model::contract::box_error::BoxError;
use crate::model::contract::task::Task;
use crate::model::project::id::ProjectId;
use crate::storage::Storage;
use sqlx::PgPool;
use sqlx::types::Json;
use std::sync::Arc;

pub struct ProjectRestoration {
    pool: Arc<PgPool>,
    storage: Arc<Storage>,
    project_id: ProjectId,
}

impl ProjectRestoration {
    pub fn new(pool: Arc<PgPool>, storage: Arc<Storage>, project_id: ProjectId) -> Self {
        Self {
            pool,
            storage,
            project_id,
        }
    }
}

#[async_trait::async_trait]
impl Task for ProjectRestoration {
    type Output = ();

    async fn perform(&self) -> Result<Self::Output, BoxError> {
        let storage_key: Option<(String,)> =
            sqlx::query_as("SELECT storage_key FROM archived_projects WHERE id = $1")
                .bind(self.project_id.id())
                .fetch_optional(self.pool.as_ref())
                .await?;

        let (storage_key,) = storage_key.ok_or(sqlx::Error::RowNotFound)?;

        let bytes = self.storage.read_bytes(&storage_key).await?;
        let payload: serde_json::Value = serde_json::from_slice(&bytes)?;

        let title = payload["title"]
            .as_str()
            .ok_or("Missing title in archive payload")?;
        let updated_at = payload["updated_at"]
            .as_str()
            .ok_or("Missing updated_at in archive payload")?;

        let stages = Json(&payload["stages"]);
        let payments = Json(&payload["payments"]);
        let tranches = Json(&payload["tranches"]);
        let comments = Json(&payload["comments"]);
        let attachments = Json(&payload["attachments"]);

        let mut tx = self.pool.begin().await?;

        sqlx::query(
            "INSERT INTO projects (id, title, updated_at)
             VALUES ($1, $2, $3::timestamptz)",
        )
        .bind(self.project_id.id())
        .bind(title)
        .bind(updated_at)
        .execute(&mut *tx)
        .await?;

        sqlx::query(
            "INSERT INTO stages (project_id, parent_position, position, title, deadline, gip_confirmed)
             SELECT $1, parent_position, position, title, deadline, gip_confirmed
             FROM jsonb_to_recordset($2) AS s(
                 parent_position INT,
                 position        INT,
                 title           VARCHAR(255),
                 deadline        TIMESTAMPTZ,
                 gip_confirmed   BOOL
             )
             ORDER BY parent_position, position",
        )
            .bind(self.project_id.id())
            .bind(stages)
            .execute(&mut *tx)
            .await?;

        sqlx::query(
            "INSERT INTO stage_payments (project_id, parent_position, stage_position, kind, cost, confirmed)
             SELECT $1, parent_position, stage_position, kind, cost, confirmed
             FROM jsonb_to_recordset($2) AS p(
                 parent_position INT,
                 stage_position  INT,
                 kind            VARCHAR(32),
                 cost            NUMERIC(17, 2),
                 confirmed       BOOL
             )",
        )
            .bind(self.project_id.id())
            .bind(payments)
            .execute(&mut *tx)
            .await?;

        sqlx::query(
            "INSERT INTO stage_payment_tranches (id, project_id, parent_position, stage_position, kind, amount, created_at, created_by)
             SELECT id, $1, parent_position, stage_position, kind, amount, created_at, created_by
             FROM jsonb_to_recordset($2) AS t(
                 id              UUID,
                 parent_position INT,
                 stage_position  INT,
                 kind            VARCHAR(32),
                 amount          NUMERIC(17, 2),
                 created_at      TIMESTAMPTZ,
                 created_by      UUID
             )",
        )
            .bind(self.project_id.id())
            .bind(tranches)
            .execute(&mut *tx)
            .await?;

        sqlx::query(
            "INSERT INTO attachments (id, project_id, parent_position, stage_position, filename, mime_type, size_bytes, is_act)
             SELECT id, $1, parent_position, stage_position, filename, mime_type, size_bytes, is_act
             FROM jsonb_to_recordset($2) AS a(
                 id              UUID,
                 parent_position INT,
                 stage_position  INT,
                 filename        VARCHAR(255),
                 mime_type       VARCHAR(127),
                 size_bytes      BIGINT,
                 is_act          BOOL
             )",
        )
            .bind(self.project_id.id())
            .bind(attachments)
            .execute(&mut *tx)
            .await?;

        sqlx::query(
            "INSERT INTO stage_comments (id, project_id, parent_position, stage_position, author_id, text, created_at, is_system, is_pinned)
             SELECT id, $1, parent_position, stage_position, author_id, text, created_at, is_system, is_pinned
             FROM jsonb_to_recordset($2) AS c(
                 id              UUID,
                 parent_position INT,
                 stage_position  INT,
                 author_id       UUID,
                 text            TEXT,
                 created_at      TIMESTAMPTZ,
                 is_system       BOOL,
                 is_pinned       BOOL
             )",
        )
            .bind(self.project_id.id())
            .bind(comments)
            .execute(&mut *tx)
            .await?;

        sqlx::query("DELETE FROM archived_projects WHERE id = $1")
            .bind(self.project_id.id())
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;

        let _ = self.storage.delete(&storage_key).await;
        Ok(())
    }
}
