ALTER TABLE stages DROP CONSTRAINT stages_pkey;
ALTER TABLE stages ADD CONSTRAINT stages_pkey PRIMARY KEY (project_id, position);