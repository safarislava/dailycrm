use crate::model::contract::box_error::BoxError;
use crate::model::contract::task::Task;
use crate::model::project::id::ProjectId;
use crate::storage::Storage;
use sqlx::PgPool;
use sqlx::types::Json;
use std::sync::Arc;

pub struct ProjectArchivation {
    pool: Arc<PgPool>,
    storage: Arc<Storage>,
    project_id: ProjectId,
}

impl ProjectArchivation {
    pub fn new(pool: Arc<PgPool>, storage: Arc<Storage>, project_id: ProjectId) -> Self {
        Self {
            pool,
            storage,
            project_id,
        }
    }
}

#[async_trait::async_trait]
impl Task for ProjectArchivation {
    type Output = ();

    async fn perform(&self) -> Result<Self::Output, BoxError> {
        let mut tx = self.pool.begin().await?;

        let row: Option<(Json<serde_json::Value>,)> = sqlx::query_as(
            "SELECT json_build_object(
                'id', p.id,
                'title', p.title,
                'updated_at', p.updated_at,
                'stages', COALESCE((
                    SELECT json_agg(
                        json_build_object(
                            'parent_position', s.parent_position,
                            'position', s.position,
                            'title', s.title,
                            'deadline', s.deadline,
                            'completed', s.completed,
                            'gip_confirmed', s.gip_confirmed,
                            'has_act', EXISTS(
                                SELECT 1 FROM attachments a
                                WHERE a.project_id = s.project_id
                                  AND a.parent_position = s.parent_position
                                  AND a.stage_position = s.position
                                  AND a.is_act = TRUE
                            ),
                            'payments', COALESCE((
                                SELECT json_agg(
                                    json_build_object(
                                        'kind', sp.kind,
                                        'cost', sp.cost,
                                        'confirmed', sp.confirmed,
                                        'tranches', COALESCE((
                                            SELECT json_agg(
                                                json_build_object(
                                                    'id', spt.id,
                                                    'amount', spt.amount,
                                                    'created_at', spt.created_at,
                                                    'created_by', spt.created_by
                                                ) ORDER BY spt.created_at
                                            )
                                            FROM stage_payment_tranches spt
                                            WHERE spt.project_id = sp.project_id
                                              AND spt.parent_position = sp.parent_position
                                              AND spt.stage_position = sp.stage_position
                                              AND spt.kind = sp.kind
                                        ), '[]'::json)
                                    )
                                )
                                FROM stage_payments sp
                                WHERE sp.project_id = s.project_id
                                  AND sp.parent_position = s.parent_position
                                  AND sp.stage_position = s.position
                            ), '[]'::json)
                        ) ORDER BY s.parent_position, s.position
                    )
                    FROM detailed_stages s
                    WHERE s.project_id = p.id
                ), '[]'::json),
                'payments', COALESCE((
                    SELECT json_agg(
                        json_build_object(
                            'parent_position', sp.parent_position,
                            'stage_position', sp.stage_position,
                            'kind', sp.kind,
                            'cost', sp.cost,
                            'confirmed', sp.confirmed
                        )
                    )
                    FROM stage_payments sp
                    WHERE sp.project_id = p.id
                ), '[]'::json),
                'tranches', COALESCE((
                    SELECT json_agg(
                        json_build_object(
                            'id', spt.id,
                            'parent_position', spt.parent_position,
                            'stage_position', spt.stage_position,
                            'kind', spt.kind,
                            'amount', spt.amount,
                            'created_at', spt.created_at,
                            'created_by', spt.created_by
                        )
                    )
                    FROM stage_payment_tranches spt
                    WHERE spt.project_id = p.id
                ), '[]'::json),
                'comments', COALESCE((
                    SELECT json_agg(
                        json_build_object(
                            'id', sc.id,
                            'parent_position', sc.parent_position,
                            'stage_position', sc.stage_position,
                            'author_id', sc.author_id,
                            'author', u.username,
                            'text', sc.text,
                            'created_at', sc.created_at,
                            'is_system', sc.is_system,
                            'is_pinned', sc.is_pinned
                        ) ORDER BY sc.created_at
                    )
                    FROM stage_comments sc
                    JOIN users u ON u.id = sc.author_id
                    WHERE sc.project_id = p.id
                ), '[]'::json),
                'attachments', COALESCE((
                    SELECT json_agg(
                        json_build_object(
                            'id', a.id,
                            'parent_position', a.parent_position,
                            'stage_position', a.stage_position,
                            'filename', a.filename,
                            'mime_type', a.mime_type,
                            'size_bytes', a.size_bytes,
                            'is_act', a.is_act
                        )
                    )
                    FROM attachments a
                    WHERE a.project_id = p.id
                ), '[]'::json)
            )
            FROM projects p
            WHERE p.id = $1",
        )
        .bind(self.project_id.id())
        .fetch_optional(&mut *tx)
        .await?;

        let (payload,) = row.ok_or(sqlx::Error::RowNotFound)?;
        let title = payload["title"]
            .as_str()
            .ok_or("Missing project title in payload")?;

        let json_bytes = serde_json::to_vec(&payload.0)?;
        let storage_key = format!("archives/{}.json", self.project_id.id());
        self.storage
            .write_bytes(&storage_key, json_bytes, "application/json")
            .await?;

        sqlx::query("INSERT INTO archived_projects (id, title, storage_key) VALUES ($1, $2, $3)")
            .bind(self.project_id.id())
            .bind(title)
            .bind(&storage_key)
            .execute(&mut *tx)
            .await?;

        let deleted = sqlx::query("DELETE FROM projects WHERE id = $1")
            .bind(self.project_id.id())
            .execute(&mut *tx)
            .await?;

        if deleted.rows_affected() == 0 {
            return Err(BoxError::from(sqlx::Error::RowNotFound));
        }

        tx.commit().await?;
        Ok(())
    }
}
