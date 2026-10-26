-- Normalize stage payments into a dedicated stage_payments table.
-- Supports arbitrary payment kinds (e.g. first_advance, second_advance, final).

CREATE TABLE stage_payments
(
    project_id      UUID        NOT NULL,
    parent_position INTEGER     NOT NULL,
    stage_position  INTEGER     NOT NULL,
    kind            VARCHAR(32) NOT NULL,
    cost            NUMERIC(17, 2),
    confirmed       BOOLEAN     NOT NULL DEFAULT FALSE,
    PRIMARY KEY (project_id, parent_position, stage_position, kind),
    FOREIGN KEY (project_id, parent_position, stage_position)
        REFERENCES stages (project_id, parent_position, position)
        ON UPDATE CASCADE
        ON DELETE CASCADE
);

-- Migrate existing advance payment data
INSERT INTO stage_payments (project_id, parent_position, stage_position, kind, cost, confirmed)
SELECT project_id, parent_position, position, 'first_advance', advance_cost, advance_confirmed
FROM stages
WHERE advance_cost IS NOT NULL
   OR advance_confirmed = TRUE;

-- Migrate existing final payment data
INSERT INTO stage_payments (project_id, parent_position, stage_position, kind, cost, confirmed)
SELECT project_id, parent_position, position, 'final', final_cost, final_confirmed
FROM stages
WHERE final_cost IS NOT NULL
   OR final_confirmed = TRUE;

-- Drop old columns from stages
DROP VIEW detailed_stages;

ALTER TABLE stages
    DROP COLUMN advance_cost;
ALTER TABLE stages
    DROP COLUMN advance_confirmed;
ALTER TABLE stages
    DROP COLUMN final_cost;
ALTER TABLE stages
    DROP COLUMN final_confirmed;

-- Recreate detailed_stages view joining stage_payments
CREATE VIEW detailed_stages AS
SELECT s.project_id,
       s.parent_position,
       s.position,
       s.title,
       s.deadline,
       s.gip_confirmed,
       p1.cost                                AS advance_cost,
       COALESCE(p1.confirmed, FALSE)          AS advance_confirmed,
       p2.cost                                AS second_advance_cost,
       COALESCE(p2.confirmed, FALSE)          AS second_advance_confirmed,
       pf.cost                                AS final_cost,
       COALESCE(pf.confirmed, FALSE)          AS final_confirmed,
       (s.gip_confirmed
           AND (p1.cost IS NULL OR p1.confirmed)
           AND (p2.cost IS NULL OR p2.confirmed)
           AND COALESCE(pf.confirmed, FALSE)
           AND EXISTS(SELECT 1
                      FROM attachments a
                      WHERE a.project_id = s.project_id
                        AND a.parent_position = s.parent_position
                        AND a.stage_position = s.position
                        AND a.is_act = TRUE)) AS completed
FROM stages s
         LEFT JOIN stage_payments p1 ON p1.project_id = s.project_id AND p1.parent_position = s.parent_position AND
                                        p1.stage_position = s.position AND p1.kind = 'first_advance'
         LEFT JOIN stage_payments p2 ON p2.project_id = s.project_id AND p2.parent_position = s.parent_position AND
                                        p2.stage_position = s.position AND p2.kind = 'second_advance'
         LEFT JOIN stage_payments pf ON pf.project_id = s.project_id AND pf.parent_position = s.parent_position AND
                                        pf.stage_position = s.position AND pf.kind = 'final';
