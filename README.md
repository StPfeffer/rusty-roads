# Rust Route Manager

This is a simple API written in [Rust](https://www.rust-lang.org/).

## How to run

This code is using a [PostgreSQL](https://www.postgresql.org/) container in [Docker](https://www.docker.com/).

### Docker

To start the PostgreSQL container:

`docker compose up postgres`

To end the container:

`docker compose down postgres`

### Cargo

After the container is running, you can execute the application using the following command:

`cargo run`

## Migrations

Generate reversible migration scripts containing both "up" and "down" SQL files:

`sqlx migrate add -r "description"`

To syncronize the database schema with the migration scripts in "`./migrations`", execute:

`sqlx migrate run`
