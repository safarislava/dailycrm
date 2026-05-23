CREATE OR REPLACE FUNCTION delete_expired_invites()
RETURNS TRIGGER AS $$
BEGIN
    DELETE FROM invites WHERE expires_at <= NOW();
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER invites_cleanup_expired
    AFTER INSERT ON invites
    FOR EACH STATEMENT
    EXECUTE FUNCTION delete_expired_invites();