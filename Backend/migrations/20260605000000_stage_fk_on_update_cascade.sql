-- Reordering a stage changes its position, which is also its identity.
-- Cascade position/parent_position changes to dependent rows automatically.
ALTER TABLE attachments DROP CONSTRAINT attachments_stage_fk;
ALTER TABLE attachments ADD CONSTRAINT attachments_stage_fk
    FOREIGN KEY (project_id, parent_position, stage_position)
    REFERENCES stages(project_id, parent_position, position)
    ON UPDATE CASCADE ON DELETE CASCADE;

ALTER TABLE stage_comments DROP CONSTRAINT stage_comments_stage_fk;
ALTER TABLE stage_comments ADD CONSTRAINT stage_comments_stage_fk
    FOREIGN KEY (project_id, parent_position, stage_position)
    REFERENCES stages(project_id, parent_position, position)
    ON UPDATE CASCADE ON DELETE CASCADE;