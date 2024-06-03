-- Add up migration script here
CREATE TABLE IF NOT EXISTS vehicles_documents
(
    id                  UUID               NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
    chassis_number      VARCHAR(17) UNIQUE NOT NULL,
    exercise_year       INTEGER            NOT NULL,
    model_year          INTEGER            NOT NULL,
    manufacture_year    INTEGER            NOT NULL,
    registration_number VARCHAR(20) UNIQUE NOT NULL,
    color               VARCHAR(60)        NOT NULL,
    make                VARCHAR(60)        NOT NULL,
    model               VARCHAR(60)        NOT NULL,
    plate               VARCHAR(60) UNIQUE NOT NULL,
    updated_at          TIMESTAMP                               DEFAULT NOW() NOT NULL,
    vehicle_id          UUID               NOT NULL
        CONSTRAINT fk_vehicles_documents_vehicle_id
            REFERENCES vehicles (id)
);
