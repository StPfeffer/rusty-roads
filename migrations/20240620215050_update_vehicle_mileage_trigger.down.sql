-- Add down migration script here
DROP TRIGGER IF EXISTS after_insert_routes ON routes;
DROP FUNCTION IF EXISTS update_vehicle_mileage;
