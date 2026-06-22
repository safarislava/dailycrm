-- Split the single cost/payment_confirmed pair into an advance payment and
-- a final payment, each with its own amount and confirmation.
DROP VIEW detailed_stages;

ALTER TABLE stages ADD COLUMN advance_cost      INTEGER;
ALTER TABLE stages ADD COLUMN advance_confirmed BOOLEAN NOT NULL DEFAULT FALSE;
ALTER TABLE stages ADD COLUMN final_cost        INTEGER;
ALTER TABLE stages ADD COLUMN final_confirmed   BOOLEAN NOT NULL DEFAULT FALSE;

UPDATE stages SET final_cost = cost, final_confirmed = payment_confirmed;

ALTER TABLE stages DROP COLUMN cost;
ALTER TABLE stages DROP COLUMN payment_confirmed;

-- Completed now requires both the advance and the final payment confirmed,
-- generalizing the old single payment_confirmed flag.
CREATE VIEW detailed_stages AS
SELECT s.*,
       (s.gip_confirmed AND s.advance_confirmed AND s.final_confirmed AND EXISTS(
           SELECT 1 FROM attachments a
           WHERE a.project_id = s.project_id
             AND a.parent_position = s.parent_position
             AND a.stage_position = s.position
             AND a.is_act = TRUE
       )) AS completed
FROM stages s;