CREATE TABLE invites (
    token      UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    created_by UUID        NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ NOT NULL DEFAULT NOW() + INTERVAL '7 days',
    used_at    TIMESTAMPTZ
);