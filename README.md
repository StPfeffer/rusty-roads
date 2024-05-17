<div align="center">

<h1 align="center">Rust Route Manager</h1>

<p align="center">
    <strong>A simple route manager application written in Rust</strong>
</p>

[![Build](https://github.com/StPfeffer/rust-route-manager/actions/workflows/build.yml/badge.svg)](https://github.com/StPfeffer/rust-route-manager/actions/workflows/build.yml)
[![Rust Analyze](https://github.com/StPfeffer/rust-route-manager/actions/workflows/rust-clippy.yml/badge.svg)](https://github.com/StPfeffer/rust-route-manager/actions/workflows/rust-clippy.yml)
[![Rust Check](https://github.com/StPfeffer/rust-route-manager/actions/workflows/rust.yml/badge.svg)](https://github.com/StPfeffer/rust-route-manager/actions/workflows/rust.yml)
[![Release](https://img.shields.io/github/v/release/StPfeffer/rust-route-manager?color=%23c694ff)](https://github.com/StPfeffer/rust-route-manager/releases/latest)

</div>

## About

This project is part of our college coursework, aiming to develop a simple route manager application. We chose the Rust programming language to add an extra layer of challenge and learning opportunity.

## Features

- **Vehicle Management**: Keep track of all vehicles, including their status and details.
- **Driver Management**: Manage driver information and credentials.
- **Driver Assignment**: Assign drivers to specific vehicles efficiently.
- **Route Control**: Monitor and manage vehicle routes for optimal performance.
- **Route Optimization**: Automatically suggest the most efficient routes based on traffic and distance.
- **Scheduling**: Create and manage schedules for drivers and vehicles.
- **Notifications**: Send alerts and notifications to drivers about route changes, vehicle maintenance, and other important updates.
- **Reporting**: Generate reports on vehicle usage, driver performance, and route efficiency.
- **GPS Integration**: Integrate with GPS systems for real-time tracking of vehicles.
- **Fuel Management**: Track fuel usage and optimize refueling schedules.
- **Maintenance Tracking**: Schedule and record vehicle maintenance and repairs.
- **User Roles and Permissions**: Define roles and permissions for different users of the application.
- **Data Import/Export**: Import and export data to and from the application in various formats (e.g., CSV, Excel).
- **Mobile Access**: Access the application from mobile devices for on-the-go management.
- **Historical Data**: Maintain a history of routes, driver assignments, and vehicle usage for analysis and reference.

## Installation

Work in progress

## Build from source

### Install Rust

Ensure you have Rust installed on your machine. If not, download and install it from [rust-lang.org](https://www.rust-lang.org/).

### Install Docker

Ensure you have Docker installed on your machine. If not, download and install ir from [docker.com](https://www.docker.com/).

#### Database Setup

After installing Docker, go to the project directory and start the PostgreSQL container:

```shell
docker compose up postgres
```

### Install Dependencies

After the container is running, you can build the application using the following command:

```shell
cargo build
```

This will install all the required dependencies to run the application.

## Environment Variables

You'll need a PostgreSQL user, password, database and URL.

Once you have it, you can copy or rename the `.env.example` file to `.env` and put it on `POSTGRES_USER`, `POSTGRES_PASSWORD`, `POSTGRES_DB` and `DATABASE_URL`.

## Running

Once you've got all things set up, you can run the following command to start the application:

```shell
cargo run
```

## License

This project is licensed under the [MIT License](https://github.com/StPfeffer/rust-route-manager/blob/main/LICENSE).
