-- Add up migration script here
CREATE TABLE IF NOT EXISTS states
(
    id         UUID         NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
    name       VARCHAR(100) NOT NULL UNIQUE,
    code       VARCHAR(2)   NOT NULL UNIQUE,
    country_id UUID         NOT NULL
        CONSTRAINT fk_states_country_id
            REFERENCES countries(id)
);
