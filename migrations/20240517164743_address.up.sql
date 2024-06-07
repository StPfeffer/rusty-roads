-- Add up migration script here
CREATE TABLE IF NOT EXISTS addresses
(
    id            UUID         NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
    address       VARCHAR(100) NOT NULL,
    number        VARCHAR(10)  NOT NULL,
    neighbourhood VARCHAR(60)  NOT NULL,
    reference     VARCHAR(60),
    complement    VARCHAR(60),
    zip_code      VARCHAR(8)   NOT NULL,
    latitude      NUMERIC(10, 7),
    longitude     NUMERIC(10, 8),
    city_id       UUID         NOT NULL
        CONSTRAINT fk_addresses_city_id
            REFERENCES cities (id)
);
