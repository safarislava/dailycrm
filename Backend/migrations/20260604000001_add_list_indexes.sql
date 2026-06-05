CREATE INDEX idx_stage_comments_stage ON stage_comments (project_id, parent_position, stage_position, created_at);

CREATE INDEX idx_attachments_stage ON attachments (project_id, parent_position, stage_position, is_act, created_at);

CREATE INDEX idx_projects_updated_at ON projects (updated_at DESC);

CREATE INDEX idx_stages_project ON stages (project_id, parent_position, position);