-- Add up migration script here
ALTER TABLE IF EXISTS routes ADD COLUMN IF NOT EXISTS driver_id UUID CONSTRAINT fk_routes_driver_id REFERENCES drivers(id);