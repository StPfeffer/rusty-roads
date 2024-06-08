-- Add up migration script here
CREATE TABLE IF NOT EXISTS cnh_types
(
    id          UUID               NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
    code        VARCHAR(10) UNIQUE NOT NULL,
    description VARCHAR(40)        NOT NULL
);

CREATE TABLE IF NOT EXISTS drivers
(
    id                  UUID                    NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
    cnh_number          VARCHAR(11)             NOT NULL,
    cnh_expiration_date DATE                    NOT NULL,
    created_at          TIMESTAMP DEFAULT NOW() NOT NULL,
    updated_at          TIMESTAMP DEFAULT NOW() NOT NULL,
    cnh_type_id         UUID                    NOT NULL
        CONSTRAINT fk_drivers_cnh_type_id
            REFERENCES cnh_types (id),
    collaborator_id     UUID UNIQUE             NOT NULL
        CONSTRAINT fk_drivers_collaborator_id
            REFERENCES collaborators (id)
);

ALTER TABLE drivers ADD CONSTRAINT unq_drivers_cnh_number UNIQUE (cnh_number);
