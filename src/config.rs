#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_maxage: i64,
    pub port: u16,
}

impl Config {
    pub fn init() -> Config {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let jwt_secret = std::env::var("JWT_SECRET_KEY").expect("JWT_SECRET_KEY must be set");
        let jwt_maxage = std::env::var("JWT_MAXAGE").expect("JWT_MAXAGE must be set");
        let port = std::env::var("PORT").unwrap_or("8080".to_string());

        Config {
            database_url,
            jwt_secret,
            // parse the String received from the environment variable to an i64
            jwt_maxage: jwt_maxage.parse::<i64>().unwrap(),
            port: port.parse::<u16>().unwrap_or(8080),
        }
    }
}
