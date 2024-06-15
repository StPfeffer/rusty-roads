-- Add down migration script here
ALTER TABLE routes DROP COLUMN IF EXISTS driver_id;
