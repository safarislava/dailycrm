DROP VIEW detailed_stages;

ALTER TABLE stages
    ALTER COLUMN advance_cost TYPE NUMERIC(17, 2);
ALTER TABLE stages
    ALTER COLUMN final_cost TYPE NUMERIC(17, 2);

CREATE VIEW detailed_stages AS
SELECT s.*,
       (s.gip_confirmed
           AND (s.advance_cost IS NULL OR s.advance_confirmed)
           AND s.final_confirmed
           AND EXISTS(SELECT 1
                      FROM attachments a
                      WHERE a.project_id = s.project_id
                        AND a.parent_position = s.parent_position
                        AND a.stage_position = s.position
                        AND a.is_act = TRUE)) AS completed
FROM stages s;
