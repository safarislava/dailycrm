ALTER TABLE attachments DROP CONSTRAINT attachments_project_id_fkey;
ALTER TABLE attachments ADD FOREIGN KEY (project_id, stage_position) REFERENCES stages(project_id, position) ON DELETE CASCADE;