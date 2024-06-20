-- Add up migration script here
CREATE OR REPLACE FUNCTION update_updated_on_table()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE OR REPLACE TRIGGER update_updated_on_table
    BEFORE UPDATE
    ON
        collaborators
    FOR EACH ROW
EXECUTE PROCEDURE update_updated_on_table();

CREATE OR REPLACE TRIGGER update_updated_on_table
    BEFORE UPDATE
    ON
        drivers
    FOR EACH ROW
EXECUTE PROCEDURE update_updated_on_table();

CREATE OR REPLACE TRIGGER update_updated_on_table
    BEFORE UPDATE
    ON
        routes
    FOR EACH ROW
EXECUTE PROCEDURE update_updated_on_table();

CREATE OR REPLACE TRIGGER update_updated_on_table
    BEFORE UPDATE
    ON
        vehicles
    FOR EACH ROW
EXECUTE PROCEDURE update_updated_on_table();

CREATE OR REPLACE TRIGGER update_updated_on_table
    BEFORE UPDATE
    ON
        vehicles_documents
    FOR EACH ROW
EXECUTE PROCEDURE update_updated_on_table();