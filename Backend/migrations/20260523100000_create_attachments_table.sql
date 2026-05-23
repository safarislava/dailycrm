CREATE TABLE attachments (
    id            UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id    UUID        NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    stage_position INT        NOT NULL,
    filename      TEXT        NOT NULL,
    mime_type     TEXT        NOT NULL,
    size_bytes    BIGINT      NOT NULL,
    object_key    TEXT        NOT NULL,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW()
);