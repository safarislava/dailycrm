CREATE TABLE user_roles (
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role    TEXT NOT NULL CHECK (role IN ('gip', 'lawyer', 'accountant')),
    PRIMARY KEY (user_id, role)
);

ALTER TABLE attachments ADD COLUMN is_act BOOLEAN NOT NULL DEFAULT FALSE;

CREATE TABLE notification_queue (
    id            UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    type          TEXT        NOT NULL CHECK (type IN ('work_complete', 'act_uploaded')),
    project_title TEXT        NOT NULL,
    stage_title   TEXT        NOT NULL,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE stages ADD COLUMN gip_confirmed     BOOLEAN NOT NULL DEFAULT FALSE;
ALTER TABLE stages ADD COLUMN act_attachment_id  UUID    REFERENCES attachments(id) ON DELETE SET NULL;
ALTER TABLE stages ADD COLUMN payment_confirmed  BOOLEAN NOT NULL DEFAULT FALSE;

UPDATE stages SET gip_confirmed = completed;

ALTER TABLE stages DROP COLUMN completed;
ALTER TABLE stages ADD COLUMN completed BOOLEAN GENERATED ALWAYS AS (
    gip_confirmed AND act_attachment_id IS NOT NULL AND payment_confirmed
) STORED;