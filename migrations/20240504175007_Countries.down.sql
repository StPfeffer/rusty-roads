-- Add down migration script here
DROP TABLE IF EXISTS countries;

DROP EXTENSION IF EXISTS "uuid-ossp";