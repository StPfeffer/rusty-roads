-- Add down migration script here
DROP TRIGGER IF EXISTS update_updated_on_table ON vehicles_documents;
DROP TRIGGER IF EXISTS update_updated_on_table ON collaborators;
DROP TRIGGER IF EXISTS update_updated_on_table ON drivers;
DROP TRIGGER IF EXISTS update_updated_on_table ON routes;
DROP TRIGGER IF EXISTS update_updated_on_table ON vehicles;

DROP FUNCTION IF EXISTS update_updated_on_table;