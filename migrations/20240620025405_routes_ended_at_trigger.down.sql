-- Add down migration script here
DROP TRIGGER IF EXISTS before_update_route_status ON routes;
DROP FUNCTION IF EXISTS update_ended_at_routes;