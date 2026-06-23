-- Make stages cascade delete when their associated project is deleted.
DO $$
DECLARE
    r record;
BEGIN
    FOR r IN
        SELECT conname FROM pg_constraint
        WHERE conrelid = 'stages'::regclass AND confrelid = 'projects'::regclass
    LOOP
        EXECUTE 'ALTER TABLE stages DROP CONSTRAINT ' || quote_ident(r.conname);
    END LOOP;
END $$;

ALTER TABLE stages
    ADD CONSTRAINT stages_project_id_fkey
    FOREIGN KEY (project_id) REFERENCES projects(id)
    ON DELETE CASCADE;
