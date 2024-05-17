mod config;
mod db;
mod dtos;
mod error;
mod models;
mod scopes;

use actix_cors::Cors;
use actix_web::{
    get, http::header, middleware::Logger, web, App, HttpResponse, HttpServer, Responder,
};
use config::Config;
use db::client::DBClient;
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct AppState {
    pub env: Config,
    pub db_client: DBClient,
}

const MAX_RETRIES: i32 = 10;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }

    dotenv().ok();

    let config = Config::init();

    println!("{}", &config.database_url);

    let pool = establish_database_connection(&config.database_url, MAX_RETRIES).await?;

    match sqlx::migrate!("./migrations").run(&pool).await {
        Ok(_) => println!("Migrations executed successfully."),
        Err(e) => eprintln!("Error executing migrations: {}", e),
    };

    let db_client = DBClient::new(pool);
    let app_state: AppState = AppState {
        env: config.clone(),
        db_client,
    };

    println!("Server is running on http://localhost:{}", config.port);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_origin("http://localhost:8000")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();

        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .wrap(cors)
            .wrap(Logger::default())
            .service(scopes::country::country_scope())
            .service(scopes::state::state_scope())
            .service(health_checker_handler)
    })
    .bind(("0.0.0.0", config.port))?
    .run()
    .await?;

    Ok(())
}

/// Establishes a connection to a PostgreSQL database using the provided URL.
///
/// # Arguments
///
/// * `database_url` - A string slice representing the URL of the PostgreSQL database.
/// * `max_retries` - An integer representing the maximum number of retries in case of connection failure.
///
/// # Returns
///
/// Returns a Result containing a PostgreSQL connection pool (`sqlx::PgPool`) if successful,
/// otherwise returns an error boxed as `Box<dyn std::error::Error>`.
///
/// # Errors
///
/// Returns an error if the connection to the database fails after the maximum number of retries
/// specified by `max_retries`.
///
/// # Examples
///
/// ```rust
/// use std::time::Duration;
///
/// // Example usage to establish a database connection with a maximum of 3 retries
/// let database_url = "postgres://username:password@localhost:5432/database_name";
/// let max_retries = 3;
/// let connection_result = establish_database_connection(database_url, max_retries).await;
///
/// match connection_result {
///     Ok(pool) => {
///         println!("Connected to the database successfully!");
///         // Further database operations using the pool
///     }
///     Err(err) => {
///         eprintln!("Failed to establish database connection: {}", err);
///         // Handle the error appropriately
///     }
/// }
/// ```
///
/// # Notes
///
/// This function retries connecting to the database with an interval of 5 seconds between retries.
///
/// # Panics
///
/// This function does not panic under normal circumstances. However, it may panic if the provided
/// `database_url` is malformed or if there is an issue with the Tokio runtime.
///
/// # Safety
///
/// This function does not contain unsafe code.
///
/// # Performance
///
/// The performance of this function depends on the network latency and the responsiveness of the
/// PostgreSQL server. Retrying the connection multiple times may impact performance if the server
/// is slow to respond or if there are network issues.
async fn establish_database_connection(
    database_url: &str,
    max_retries: i32,
) -> Result<sqlx::PgPool, Box<dyn std::error::Error>> {
    let mut retries = 0;
    loop {
        match PgPoolOptions::new()
            .max_connections(10)
            .connect(database_url)
            .await
        {
            Ok(pool) => return Ok(pool),
            Err(err) => {
                eprintln!("Error connecting to the database: {}", err);
                retries += 1;

                if retries >= max_retries {
                    return Err(Box::new(err));
                }

                println!("Retrying database connection...");
                tokio::time::sleep(Duration::from_secs(5)).await; // Wait for 5 seconds before retrying
            }
        }
    }
}

/// Handles HTTP GET requests to '/api/v1/healthchecker', providing a simple health check response.
///
/// # Returns
///
/// Returns an `impl Responder` representing an HTTP response with status code 200 (OK) and a JSON payload
/// containing a status message indicating success and a custom message.
///
/// # Examples
///
/// ```rust
/// use actix_web::{test, App};
///
/// #[actix_rt::test]
/// async fn test_health_checker_handler() {
///     let mut app = test::init_service(App::new().service(health_checker_handler)).await;
///     let req = test::TestRequest::get().uri("/api/v1/healthchecker").to_request();
///     let resp = test::call_service(&mut app, req).await;
///
///     assert!(resp.status().is_success());
///     let body = test::read_body(resp).await;
///     assert_eq!(body, "{\"status\":\"success\",\"message\":\"Rust Route Manager\"}");
/// }
/// ```
///
/// # Safety
///
/// This function does not contain unsafe code.
///
/// # Performance
///
/// The performance of this function is dependent on the performance of the Actix Web framework
/// and the overhead associated with JSON serialization and HTTP response handling.
#[get("/api/v1/healthchecker")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "Rust Route Manager";

    HttpResponse::Ok().json(serde_json::json!({"status": "success", "message": MESSAGE}))
}
