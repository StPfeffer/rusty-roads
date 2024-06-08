use in_container::in_container;

/// Represents the configuration settings for the application.
#[derive(Debug, Clone)]
pub struct Config {
    /// The URL of the database.
    pub database_url: String,
    /// The secret key used for JSON Web Token (JWT) generation and validation.
    pub jwt_secret: String,
    /// The maximum age (in seconds) for JWT tokens.
    pub jwt_maxage: i64,
    /// The port on which the application listens for incoming connections.
    pub port: u16,
}

impl Config {
    /// Initializes the application configuration based on environment variables.
    ///
    /// # Panics
    ///
    /// Panics if any of the required environment variables (`DATABASE_URL`, `JWT_SECRET_KEY`, `JWT_MAXAGE`)
    /// are not set or if there is an error while parsing the `JWT_MAXAGE` value.
    ///
    /// # Returns
    ///
    /// Returns a `Config` instance initialized with values from environment variables.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use your_module_name::Config;
    ///
    /// // Initialize the configuration settings
    /// let config = Config::init();
    /// println!("{:?}", config);
    /// ```
    pub fn init() -> Config {
        let mut database_url;

        if (in_container()) {
            database_url =
                std::env::var("DATABASE_URL_DOCKER").expect("DATABASE_URL_DOCKER must be set");
        } else {
            database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        }

        let jwt_secret = std::env::var("JWT_SECRET_KEY").expect("JWT_SECRET_KEY must be set");
        let jwt_maxage = std::env::var("JWT_MAXAGE").expect("JWT_MAXAGE must be set");

        Config {
            database_url,
            jwt_secret,
            jwt_maxage: jwt_maxage.parse::<i64>().unwrap(),
            port: 8000, // Default port
        }
    }
}
