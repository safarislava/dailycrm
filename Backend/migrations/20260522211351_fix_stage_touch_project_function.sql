CREATE OR REPLACE FUNCTION stage_touch_project()
    RETURNS TRIGGER AS $$
BEGIN
    UPDATE projects
    SET updated_at = NOW()
    WHERE id = NEW.project_id;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;
