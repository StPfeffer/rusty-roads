-- Add up migration script here
CREATE TABLE IF NOT EXISTS route_status
(
    id          UUID               NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
    code        VARCHAR(20) UNIQUE NOT NULL,
    description VARCHAR(60)        NOT NULL
);
