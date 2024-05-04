use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::models::Country;

#[derive(Debug, Clone)]
pub struct DBClient {
    pool: Pool<Postgres>,
}

impl DBClient {
    pub fn new(pool: Pool<Postgres>) -> Self {
        DBClient { pool }
    }
}

#[async_trait]
pub trait CountryExt {
    async fn get_country(
        &self,
        country_id: Option<Uuid>,
        name: Option<&str>,
        alpha_2: Option<&str>,
        alpha_3: Option<&str>,
        numeric_3: Option<&str>,
    ) -> Result<Option<Country>, sqlx::Error>;

    async fn list_countries(&self, page: u32, limit: usize) -> Result<Vec<Country>, sqlx::Error>;

    async fn save_country<T: Into<String> + Send>(
        &self,
        name: T,
        alpha_2: T,
        alpha_3: T,
        numeric_3: T,
    ) -> Result<Country, sqlx::Error>;

    async fn delete_country(
        &self,
        country_id: Option<Uuid>,
    ) -> Result<Option<Country>, sqlx::Error>;
}

#[async_trait]
impl CountryExt for DBClient {
    async fn get_country(
        &self,
        country_id: Option<Uuid>,
        name: Option<&str>,
        alpha_2: Option<&str>,
        alpha_3: Option<&str>,
        numeric_3: Option<&str>,
    ) -> Result<Option<Country>, sqlx::Error> {
        let mut country: Option<Country> = None;

        if let Some(country_id) = country_id {
            country = sqlx::query_as!(
                Country,
                r#"SELECT * FROM countries WHERE id = $1"#,
                country_id
            )
            .fetch_optional(&self.pool)
            .await?;
        } else if let Some(name) = name {
            country = sqlx::query_as!(
                Country,
                r#"SELECT * FROM countries WHERE name = $1"#,
                name
            )
            .fetch_optional(&self.pool)
            .await?;
        } else if let Some(alpha_2) = alpha_2 {
            country = sqlx::query_as!(
                Country,
                r#"SELECT * FROM countries WHERE alpha_2 = $1"#,
                alpha_2
            )
            .fetch_optional(&self.pool)
            .await?;
        } else if let Some(alpha_3) = alpha_3 {
            country = sqlx::query_as!(
                Country,
                r#"SELECT * FROM countries WHERE alpha_3 = $1"#,
                alpha_3
            )
            .fetch_optional(&self.pool)
            .await?;
        } else if let Some(numeric_3) = numeric_3 {
            country = sqlx::query_as!(Country, r#"SELECT * FROM countries WHERE numeric_3 = $1"#, numeric_3) 
                .fetch_optional(&self.pool)
                .await?;
        }

        Ok(country)
    }

    async fn list_countries(&self, page: u32, limit: usize) -> Result<Vec<Country>, sqlx::Error> {
        let offset = (page - 1) * limit as u32;

        let countries = sqlx::query_as!(
            Country,
            r#"SELECT id, name, alpha_2, alpha_3, numeric_3 FROM countries LIMIT $1 OFFSET $2"#,
            limit as i64,
            offset as i64
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(countries)
    }

    async fn save_country<T: Into<String> + Send>(
        &self,
        name: T,
        alpha_2: T,
        alpha_3: T,
        numeric_3: T,
    ) -> Result<Country, sqlx::Error> {
        let country = sqlx::query_as!(
            Country,
            r#"INSERT INTO countries (name, alpha_2, alpha_3, numeric_3) VALUES ($1, $2, $3, $4) RETURNING *"#,
            &name.into(),
            &alpha_2.into(),
            &alpha_3.into(),
            &numeric_3.into(),
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(country)
    }

    async fn delete_country(
        &self,
        country_id: Option<Uuid>,
    ) -> Result<Option<Country>, sqlx::Error> {
        let mut country = None;

        if let Some(country_id) = country_id {
            country = sqlx::query_as!(
                Country,
                r#"DELETE FROM countries WHERE id = $1 RETURNING *"#,
                country_id
            )
            .fetch_optional(&self.pool)
            .await?;
        }

        Ok(country)
    }
}
