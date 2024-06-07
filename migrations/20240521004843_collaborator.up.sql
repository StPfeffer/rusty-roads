-- Add up migration script here
CREATE TABLE IF NOT EXISTS collaborators
(
    id         UUID                    NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
    name       VARCHAR(100)            NOT NULL,
    email      VARCHAR(150) UNIQUE     NOT NULL,
    cpf        VARCHAR(11)  UNIQUE     NOT NULL,
    rg         VARCHAR(9)              NOT NULL,
    created_at TIMESTAMP DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMP DEFAULT NOW() NOT NULL
);
