use async_trait::async_trait;
use uuid::Uuid;

use crate::models::vehicle::Vehicle;

use super::client::DBClient;

#[async_trait]
pub trait VehicleExt {
    async fn get_vehicle(&self, vehicle_id: Option<Uuid>) -> Result<Option<Vehicle>, sqlx::Error>;

    async fn list_vehicles(&self, page: u32, limit: usize) -> Result<Vec<Vehicle>, sqlx::Error>;

    async fn save_vehicle<T: Into<String> + Send>(
        &self,
        name: T,
        initial_mileage: i32,
        actual_mileage: i32,
    ) -> Result<Vehicle, sqlx::Error>;

    async fn delete_vehicle(
        &self,
        vehicle_id: Option<Uuid>,
    ) -> Result<Option<Vehicle>, sqlx::Error>;
}

#[async_trait]
impl VehicleExt for DBClient {
    async fn get_vehicle(&self, vehicle_id: Option<Uuid>) -> Result<Option<Vehicle>, sqlx::Error> {
        if let Some(vehicle_id) = vehicle_id {
            let vehicle = sqlx::query_as!(
                Vehicle,
                r#"SELECT * FROM vehicles WHERE id = $1"#,
                vehicle_id
            )
            .fetch_optional(&self.pool)
            .await?;
            return Ok(vehicle);
        }

        Ok(None)
    }

    async fn list_vehicles(&self, page: u32, limit: usize) -> Result<Vec<Vehicle>, sqlx::Error> {
        let offset = (page - 1) * limit as u32;

        let vehicles = sqlx::query_as!(
            Vehicle,
            r#"SELECT * FROM vehicles LIMIT $1 OFFSET $2"#,
            limit as i64,
            offset as i64
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(vehicles)
    }

    async fn save_vehicle<T: Into<String> + Send>(
        &self,
        name: T,
        initial_mileage: i32,
        actual_mileage: i32,
    ) -> Result<Vehicle, sqlx::Error> {
        let vehicle = sqlx::query_as!(
            Vehicle,
            r#"INSERT INTO vehicles (name, initial_mileage, actual_mileage) VALUES ($1, $2, $3) RETURNING *"#,
            &name.into(),
            &initial_mileage,
            &actual_mileage
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(vehicle)
    }

    async fn delete_vehicle(
        &self,
        vehicle_id: Option<Uuid>,
    ) -> Result<Option<Vehicle>, sqlx::Error> {
        let mut vehicle = None;

        if let Some(vehicle_id) = vehicle_id {
            vehicle = sqlx::query_as!(
                Vehicle,
                r#"DELETE FROM vehicles WHERE id = $1 RETURNING *"#,
                vehicle_id
            )
            .fetch_optional(&self.pool)
            .await?;
        }

        Ok(vehicle)
    }
}
