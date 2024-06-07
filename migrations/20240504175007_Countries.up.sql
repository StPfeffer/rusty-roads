-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS countries
(
    id        UUID         NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
    name      VARCHAR(100) NOT NULL UNIQUE,
    alpha_2   VARCHAR(2)   NOT NULL UNIQUE,
    alpha_3   VARCHAR(3)   NOT NULL UNIQUE,
    numeric_3 VARCHAR(3)   NOT NULL UNIQUE
);
