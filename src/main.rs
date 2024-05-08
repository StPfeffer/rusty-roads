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

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }

    dotenv().ok();

    let config = Config::init();

    println!("{}", &config.database_url);

    let pool = establish_database_connection(&config.database_url).await?;

    match sqlx::migrate!("./migrations").run(&pool).await {
        Ok(_) => println!("Migrations executed successfully."),
        Err(e) => eprintln!("Error executing migrations: {}", e),
    };

    let db_client = DBClient::new(pool);
    let app_state: AppState = AppState {
        env: config.clone(),
        db_client,
    };

    println!(
        "{}",
        format!("Server is running on http://localhost:{}", config.port)
    );

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
            .service(health_checker_handler)
    })
    .bind(("0.0.0.0", config.port))?
    .run()
    .await?;

    Ok(())
}

async fn establish_database_connection(
    database_url: &str,
) -> Result<sqlx::PgPool, Box<dyn std::error::Error>> {
    let max_retries = 10;
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

#[get("/api/healthchecker")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "Rust Route Manager";

    HttpResponse::Ok().json(serde_json::json!({"status": "success", "message": MESSAGE}))
}
