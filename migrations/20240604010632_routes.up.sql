-- Add up migration script here
CREATE TABLE IF NOT EXISTS routes
(
    id                 UUID                         NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
    started_at         TIMESTAMP      DEFAULT NOW() NOT NULL,
    ended_at           TIMESTAMP,
    total_distance     NUMERIC(10, 2) DEFAULT 0.00  NOT NULL,
    created_at         TIMESTAMP      DEFAULT NOW() NOT NULL,
    updated_at         TIMESTAMP      DEFAULT NOW() NOT NULL,
    initial_lat        NUMERIC(10, 8)               NOT NULL,
    initial_long       NUMERIC(11, 8)               NOT NULL,
    final_lat          NUMERIC(10, 8)               NOT NULL,
    final_long         NUMERIC(11, 8)               NOT NULL,
    initial_address_id UUID
        CONSTRAINT fk_routes_initial_address_id
            REFERENCES addresses (id),
    final_address_id   UUID
        CONSTRAINT fk_routes_final_address_id
            REFERENCES addresses (id),
    vehicle_id         UUID                         NOT NULL
        CONSTRAINT fk_routes_vehicle_id
            REFERENCES vehicles (id)
);
