use async_trait::async_trait;
use uuid::Uuid;

use crate::models::city::City;

use super::client::DBClient;

#[async_trait]
pub trait CityExt {
    async fn get_city(
        &self,
        city_id: Option<Uuid>,
        code: Option<&str>,
    ) -> Result<Option<City>, sqlx::Error>;

    async fn list_cities(&self, page: u32, limit: usize) -> Result<Vec<City>, sqlx::Error>;

    async fn save_city<T: Into<String> + Send>(
        &self,
        name: T,
        code: T,
        state_id: T,
    ) -> Result<City, sqlx::Error>;

    async fn delete_city(&self, city_id: Option<Uuid>) -> Result<Option<City>, sqlx::Error>;
}

#[async_trait]
impl CityExt for DBClient {
    async fn get_city(
        &self,
        city_id: Option<Uuid>,
        code: Option<&str>,
    ) -> Result<Option<City>, sqlx::Error> {
        let mut city: Option<City> = None;

        if let Some(city_id) = city_id {
            city = sqlx::query_as!(City, r#"SELECT * FROM cities WHERE id = $1"#, city_id)
                .fetch_optional(&self.pool)
                .await?;
        } else if let Some(code) = code {
            city = sqlx::query_as!(City, r#"SELECT * FROM cities WHERE code = $1"#, code)
                .fetch_optional(&self.pool)
                .await?;
        }

        Ok(city)
    }

    async fn list_cities(&self, page: u32, limit: usize) -> Result<Vec<City>, sqlx::Error> {
        let offset = (page - 1) * limit as u32;

        let cities = sqlx::query_as!(
            City,
            r#"SELECT * FROM cities LIMIT $1 OFFSET $2"#,
            limit as i64,
            offset as i64
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(cities)
    }

    async fn save_city<T: Into<String> + Send>(
        &self,
        name: T,
        code: T,
        state_id: T,
    ) -> Result<City, sqlx::Error> {
        let city = sqlx::query_as!(
            City,
            r#"INSERT INTO cities (name, code, state_id) VALUES ($1, $2, $3) RETURNING *"#,
            &name.into(),
            &code.into(),
            Uuid::parse_str(&state_id.into()).unwrap(),
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(city)
    }

    async fn delete_city(&self, city_id: Option<Uuid>) -> Result<Option<City>, sqlx::Error> {
        let mut city = None;

        if let Some(city_id) = city_id {
            city = sqlx::query_as!(
                City,
                r#"DELETE FROM cities WHERE id = $1 RETURNING *"#,
                city_id
            )
            .fetch_optional(&self.pool)
            .await?;
        }

        Ok(city)
    }
}
