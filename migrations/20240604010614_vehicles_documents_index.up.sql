-- Add up migration script here
CREATE UNIQUE INDEX IF NOT EXISTS unq_vehicles_documents_vehicle_id ON vehicles_documents(vehicle_id);
