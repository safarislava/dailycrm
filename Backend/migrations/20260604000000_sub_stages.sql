-- Drop dependent FKs first (they reference stages_pkey index)
DO $$
DECLARE r record;
BEGIN
    FOR r IN
        SELECT conname FROM pg_constraint
        WHERE conrelid = 'attachments'::regclass AND confrelid = 'stages'::regclass
    LOOP
        EXECUTE 'ALTER TABLE attachments DROP CONSTRAINT ' || quote_ident(r.conname);
    END LOOP;
END $$;

DO $$
DECLARE r record;
BEGIN
    FOR r IN
        SELECT conname FROM pg_constraint
        WHERE conrelid = 'stage_comments'::regclass AND confrelid = 'stages'::regclass
    LOOP
        EXECUTE 'ALTER TABLE stage_comments DROP CONSTRAINT ' || quote_ident(r.conname);
    END LOOP;
END $$;

-- Now safe to rebuild stages PK
ALTER TABLE stages DROP CONSTRAINT stages_pkey;
ALTER TABLE stages ADD COLUMN parent_position INTEGER NOT NULL DEFAULT 0;
ALTER TABLE stages ADD CONSTRAINT stages_pkey PRIMARY KEY (project_id, parent_position, position);

-- Cascade-delete sub-stages when their parent is deleted
CREATE OR REPLACE FUNCTION delete_sub_stages() RETURNS TRIGGER AS $$
BEGIN
    IF OLD.parent_position = 0 THEN
        DELETE FROM stages WHERE project_id = OLD.project_id AND parent_position = OLD.position;
    END IF;
    RETURN OLD;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER cascade_delete_sub_stages
BEFORE DELETE ON stages
FOR EACH ROW EXECUTE FUNCTION delete_sub_stages();

-- Add parent_position to attachments and restore 3-column FK
ALTER TABLE attachments ADD COLUMN parent_position INTEGER NOT NULL DEFAULT 0;
ALTER TABLE attachments ADD CONSTRAINT attachments_stage_fk
    FOREIGN KEY (project_id, parent_position, stage_position)
    REFERENCES stages(project_id, parent_position, position)
    ON DELETE CASCADE;

-- Add parent_position to stage_comments and restore 3-column FK
ALTER TABLE stage_comments ADD COLUMN parent_position INTEGER NOT NULL DEFAULT 0;
ALTER TABLE stage_comments ADD CONSTRAINT stage_comments_stage_fk
    FOREIGN KEY (project_id, parent_position, stage_position)
    REFERENCES stages(project_id, parent_position, position)
    ON DELETE CASCADE;