-- Add up migration script here
CREATE OR REPLACE FUNCTION update_vehicle_mileage() RETURNS TRIGGER AS $$
BEGIN
    UPDATE vehicles
    SET actual_mileage = actual_mileage + NEW.total_distance
    WHERE id = NEW.vehicle_id;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE TRIGGER after_insert_routes
AFTER INSERT ON routes
FOR EACH ROW
EXECUTE FUNCTION update_vehicle_mileage();