ALTER TABLE projects
    ADD COLUMN updated_at TIMESTAMP NOT NULL DEFAULT NOW();

CREATE OR REPLACE FUNCTION project_set_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER projects_set_updated_at
    BEFORE UPDATE ON projects
    FOR EACH ROW
    EXECUTE FUNCTION project_set_updated_at();

CREATE OR REPLACE FUNCTION stage_touch_project()
RETURNS TRIGGER AS $$
BEGIN
    UPDATE projects
    SET updated_at = NOW()
    WHERE id = COALESCE(NEW.project_id, OLD.project_id);
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER stages_after_insert_touch_project
    AFTER INSERT ON stages FOR EACH ROW
    EXECUTE FUNCTION stage_touch_project();

CREATE TRIGGER stages_after_update_touch_project
    AFTER UPDATE ON stages FOR EACH ROW
    EXECUTE FUNCTION stage_touch_project();

CREATE TRIGGER stages_after_delete_touch_project
    AFTER DELETE ON stages FOR EACH ROW
    EXECUTE FUNCTION stage_touch_project();