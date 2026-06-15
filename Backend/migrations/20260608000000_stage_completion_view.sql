-- Single source of truth for the "completed" rule.
-- A stage is completed when it is gip_confirmed, payment_confirmed,
-- and has at least one uploaded act. Multiple acts live in `attachments`,
-- so this cannot be a STORED generated column (it references another table);
-- a view keeps the rule in one place and is inlined by the planner.
CREATE VIEW detailed_stages AS
SELECT s.*,
       (s.gip_confirmed AND s.payment_confirmed AND EXISTS(
           SELECT 1 FROM attachments a
           WHERE a.project_id = s.project_id
             AND a.parent_position = s.parent_position
             AND a.stage_position = s.position
             AND a.is_act = TRUE
       )) AS completed
FROM stages s;