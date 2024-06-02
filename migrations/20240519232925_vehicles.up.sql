-- Add up migration script here
CREATE TABLE vehicles
(
    id              UUID                    NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
    name            VARCHAR(60)             NOT NULL,
    initial_mileage INTEGER                 NOT NULL,
    actual_mileage  INTEGER                 NOT NULL,
    created_at      TIMESTAMP DEFAULT NOW() NOT NULL,
    updated_at      TIMESTAMP DEFAULT NOW() NOT NULL
);
