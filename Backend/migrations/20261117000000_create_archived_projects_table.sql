CREATE TABLE archived_projects
(
    id          UUID PRIMARY KEY,
    title       VARCHAR(255) NOT NULL,
    archived_at TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    storage_key VARCHAR(255) NOT NULL
);
