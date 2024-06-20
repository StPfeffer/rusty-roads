-- Add up migration script here
CREATE OR REPLACE FUNCTION update_ended_at_routes()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.status_id = (SELECT id FROM route_status WHERE code = 'FINISHED') THEN
        NEW.ended_at = NOW();
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE 'plpgsql';

CREATE OR REPLACE TRIGGER before_update_route_status
BEFORE UPDATE ON routes
FOR EACH ROW
WHEN (OLD.status_id IS DISTINCT FROM NEW.status_id)
EXECUTE PROCEDURE update_ended_at_routes();