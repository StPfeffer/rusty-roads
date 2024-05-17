use sqlx::{Pool, Postgres};

/// Represents a client for interacting with a PostgreSQL database.
#[derive(Debug, Clone)]
pub struct DBClient {
    /// The database connection pool.
    pub pool: Pool<Postgres>,
}

impl DBClient {
    /// Creates a new instance of `DBClient` with the provided database connection pool.
    ///
    /// # Arguments
    ///
    /// * `pool` - A `sqlx::Pool<Postgres>` representing the connection pool to the PostgreSQL database.
    ///
    /// # Returns
    ///
    /// Returns a new instance of `DBClient` initialized with the given database connection pool.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use sqlx::{Pool, Postgres};
    /// use your_module_name::DBClient;
    ///
    /// // Assuming `pool` is a valid database connection pool
    /// let db_client = DBClient::new(pool);
    /// ```
    pub fn new(pool: Pool<Postgres>) -> Self {
        DBClient { pool }
    }
}
