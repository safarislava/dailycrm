-- Sending a comment did not bump the project's updated_at, since stage_touch_project()
-- was only wired up to the stages table. Reuse the same function for stage_comments,
-- which also carries a project_id column.
CREATE TRIGGER stage_comments_after_insert_touch_project
    AFTER INSERT ON stage_comments FOR EACH ROW
    EXECUTE FUNCTION stage_touch_project();