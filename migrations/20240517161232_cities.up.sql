-- Add up migration script here
CREATE TABLE IF NOT EXISTS cities
(
    id         UUID         NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
    name       VARCHAR(100) NOT NULL,
    code       VARCHAR(7)   NOT NULL UNIQUE,
    state_id   UUID         NOT NULL
        CONSTRAINT fk_cities_state_id
            REFERENCES states(id)
);
