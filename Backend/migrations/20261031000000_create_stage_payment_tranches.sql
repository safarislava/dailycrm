-- Create stage_payment_tranches table for storing partial stage payments (tranches).
CREATE TABLE stage_payment_tranches
(
    id              UUID PRIMARY KEY        DEFAULT gen_random_uuid(),
    project_id      UUID           NOT NULL,
    parent_position INTEGER        NOT NULL,
    stage_position  INTEGER        NOT NULL,
    kind            VARCHAR(32)    NOT NULL,
    amount          NUMERIC(17, 2) NOT NULL CHECK (amount > 0),
    created_at      TIMESTAMPTZ    NOT NULL DEFAULT NOW(),
    created_by      UUID           REFERENCES users (id) ON DELETE SET NULL,
    FOREIGN KEY (project_id, parent_position, stage_position, kind)
        REFERENCES stage_payments (project_id, parent_position, stage_position, kind)
        ON UPDATE CASCADE
        ON DELETE CASCADE
);

CREATE INDEX idx_tranches_stage_payment
    ON stage_payment_tranches (project_id, parent_position, stage_position, kind);

-- Seed tranches for existing confirmed payments that have a cost
INSERT INTO stage_payment_tranches (project_id, parent_position, stage_position, kind, amount, created_at)
SELECT project_id, parent_position, stage_position, kind, cost, NOW()
FROM stage_payments
WHERE confirmed = TRUE
  AND cost IS NOT NULL
  AND cost > 0;
