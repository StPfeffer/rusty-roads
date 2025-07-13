use async_trait::async_trait;
use chrono::NaiveDate;
use sqlx::Error;
use uuid::Uuid;

use super::client::DBClient;
use crate::models::driver::{CnhType, Driver};

#[async_trait]
pub trait DriverExt {
    async fn get_driver(
        &self,
        driver_id: Option<Uuid>,
        cnh_number: Option<String>,
        collaborator_id: Option<Uuid>,
    ) -> Result<Option<Driver>, sqlx::Error>;

    async fn list_drivers(&self, page: u32, limit: usize) -> Result<Vec<Driver>, sqlx::Error>;

    async fn save_driver<T: Into<String> + Send>(
        &self,
        cnh_number: T,
        cnh_expiration_date: NaiveDate,
        cnh_type_id: T,
        collaborator_id: T,
    ) -> Result<Driver, sqlx::Error>;

    async fn update_driver<T: Into<String> + Send>(
        &self,
        driver_id: Option<Uuid>,
        collaborator_id: Option<Uuid>,
        cnh_number: T,
        cnh_expiration_date: NaiveDate,
        cnh_type_id: T,
    ) -> Result<Driver, sqlx::Error>;

    async fn delete_driver(
        &self,
        driver_id: Option<Uuid>,
        collaborator_id: Option<Uuid>,
    ) -> Result<Option<Driver>, sqlx::Error>;
}

#[async_trait]
impl DriverExt for DBClient {
    async fn get_driver(
        &self,
        driver_id: Option<Uuid>,
        cnh_number: Option<String>,
        collaborator_id: Option<Uuid>,
    ) -> Result<Option<Driver>, sqlx::Error> {
        let mut driver: Option<Driver> = None;

        if let Some(driver_id) = driver_id {
            driver = sqlx::query_as!(Driver, r#"SELECT * FROM drivers WHERE id = $1"#, driver_id)
                .fetch_optional(&self.pool)
                .await?;
        } else if let Some(cnh_number) = cnh_number {
            driver = sqlx::query_as!(
                Driver,
                r#"SELECT * FROM drivers WHERE cnh_number = $1"#,
                cnh_number
            )
            .fetch_optional(&self.pool)
            .await?;
        } else if let Some(collaborator_id) = collaborator_id {
            driver = sqlx::query_as!(
                Driver,
                r#"SELECT * FROM drivers WHERE collaborator_id = $1"#,
                collaborator_id
            )
            .fetch_optional(&self.pool)
            .await?;
        }

        Ok(driver)
    }

    async fn list_drivers(&self, page: u32, limit: usize) -> Result<Vec<Driver>, sqlx::Error> {
        let offset = (page - 1) * limit as u32;

        let drivers = sqlx::query_as!(
            Driver,
            r#"SELECT * FROM drivers LIMIT $1 OFFSET $2"#,
            limit as i64,
            offset as i64
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(drivers)
    }

    async fn save_driver<T: Into<String> + Send>(
        &self,
        cnh_number: T,
        cnh_expiration_date: NaiveDate,
        cnh_type_id: T,
        collaborator_id: T,
    ) -> Result<Driver, sqlx::Error> {
        let cnh_type_id = Uuid::parse_str(&cnh_type_id.into())
            .map_err(|e| Error::Protocol(format!("Failed to parse cnh_type_id: {e}")))?;

        let collaborator_id = Uuid::parse_str(&collaborator_id.into())
            .map_err(|e| Error::Protocol(format!("Failed to parse collaborator_id: {e}")))?;

        let driver = sqlx::query_as!(
            Driver,
            r#"INSERT INTO drivers (cnh_number, cnh_expiration_date, cnh_type_id, collaborator_id) VALUES ($1, $2, $3, $4) RETURNING *"#,
            cnh_number.into(),
            cnh_expiration_date.into(),
            &cnh_type_id,
            &collaborator_id,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(driver)
    }

    async fn update_driver<T: Into<String> + Send>(
        &self,
        driver_id: Option<Uuid>,
        collaborator_id: Option<Uuid>,
        cnh_number: T,
        cnh_expiration_date: NaiveDate,
        cnh_type_id: T,
    ) -> Result<Driver, sqlx::Error> {
        let driver: Driver = match (driver_id, collaborator_id) {
            (Some(driver_id), _) => {
                sqlx::query_as!(
                    Driver,
                    r#"UPDATE drivers SET cnh_number = $2, cnh_expiration_date = $3, cnh_type_id = $4 WHERE id = $1 RETURNING *;"#,
                    &driver_id,
                    &cnh_number.into(),
                    &cnh_expiration_date,
                    Uuid::parse_str(&cnh_type_id.into()).unwrap()
                )
                .fetch_one(&self.pool)
                .await?
            }
            (None, Some(collaborator_id)) => {
                sqlx::query_as!(
                    Driver,
                    r#"UPDATE drivers SET cnh_number = $2, cnh_expiration_date = $3, cnh_type_id = $4 WHERE collaborator_id = $1 RETURNING *;"#,
                    &collaborator_id,
                    &cnh_number.into(),
                    &cnh_expiration_date,
                    Uuid::parse_str(&cnh_type_id.into()).unwrap()
                )
                .fetch_one(&self.pool)
                .await?
            }
            _ => {
                return Err(sqlx::Error::RowNotFound);
            }
        };

        Ok(driver)
    }

    async fn delete_driver(
        &self,
        driver_id: Option<Uuid>,
        collaborator_id: Option<Uuid>,
    ) -> Result<Option<Driver>, sqlx::Error> {
        let mut driver = None;

        if let Some(driver_id) = driver_id {
            driver = sqlx::query_as!(
                Driver,
                r#"DELETE FROM drivers WHERE id = $1 RETURNING *"#,
                driver_id
            )
            .fetch_optional(&self.pool)
            .await?;
        } else if let Some(collaborator_id) = collaborator_id {
            driver = sqlx::query_as!(
                Driver,
                r#"DELETE FROM drivers WHERE collaborator_id = $1 RETURNING *"#,
                collaborator_id
            )
            .fetch_optional(&self.pool)
            .await?;
        }

        Ok(driver)
    }
}

#[async_trait]
pub trait CnhTypeExt {
    async fn get_cnh_type(
        &self,
        cnh_type_id: Option<Uuid>,
        code: Option<String>,
    ) -> Result<Option<CnhType>, sqlx::Error>;

    async fn list_cnh_type(&self, page: u32, limit: usize) -> Result<Vec<CnhType>, sqlx::Error>;
}

#[async_trait]
impl CnhTypeExt for DBClient {
    async fn get_cnh_type(
        &self,
        cnh_type_id: Option<Uuid>,
        code: Option<String>,
    ) -> Result<Option<CnhType>, sqlx::Error> {
        let mut cnh_type = None;

        if let Some(cnh_type_id) = cnh_type_id {
            cnh_type = sqlx::query_as!(
                CnhType,
                r#"SELECT * FROM cnh_types WHERE id = $1"#,
                cnh_type_id
            )
            .fetch_optional(&self.pool)
            .await?;
        } else if let Some(code) = code {
            cnh_type = sqlx::query_as!(CnhType, r#"SELECT * FROM cnh_types WHERE code = $1"#, code)
                .fetch_optional(&self.pool)
                .await?;
        }

        Ok(cnh_type)
    }

    async fn list_cnh_type(&self, page: u32, limit: usize) -> Result<Vec<CnhType>, sqlx::Error> {
        let offset = (page - 1) * limit as u32;

        let cnh_types = sqlx::query_as!(
            CnhType,
            r#"SELECT * FROM cnh_types LIMIT $1 OFFSET $2"#,
            limit as i64,
            offset as i64
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(cnh_types)
    }
}
