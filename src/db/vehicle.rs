use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    dtos::vehicle::SaveVehicleDocumentParamsDTO,
    models::vehicle::{Vehicle, VehicleDocument},
};

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

#[async_trait]
pub trait VehicleDocumentExt {
    async fn get_vehicle_document(
        &self,
        document_id: Option<Uuid>,
        vehicle_id: Option<Uuid>,
        chassis_number: Option<String>,
        registration_number: Option<String>,
        plate: Option<String>,
    ) -> Result<Option<VehicleDocument>, sqlx::Error>;

    async fn list_vehicle_documents(
        &self,
        page: u32,
        limit: usize,
    ) -> Result<Vec<VehicleDocument>, sqlx::Error>;

    async fn save_vehicle_document<T: Into<String> + Send>(
        &self,
        params: SaveVehicleDocumentParamsDTO<T>,
    ) -> Result<VehicleDocument, sqlx::Error>;

    async fn delete_vehicle_document(
        &self,
        document_id: Option<Uuid>,
        vehicle_id: Option<Uuid>,
    ) -> Result<Option<VehicleDocument>, sqlx::Error>;
}

#[async_trait]
impl VehicleDocumentExt for DBClient {
    async fn get_vehicle_document(
        &self,
        document_id: Option<Uuid>,
        vehicle_id: Option<Uuid>,
        chassis_number: Option<String>,
        registration_number: Option<String>,
        plate: Option<String>,
    ) -> Result<Option<VehicleDocument>, sqlx::Error> {
        let mut document = None;

        if let Some(vehicle_id) = vehicle_id {
            document = sqlx::query_as!(
                VehicleDocument,
                r#"SELECT * FROM vehicles_documents WHERE vehicle_id = $1"#,
                vehicle_id
            )
            .fetch_optional(&self.pool)
            .await?;
        } else if let Some(document_id) = document_id {
            document = sqlx::query_as!(
                VehicleDocument,
                r#"SELECT * FROM vehicles_documents WHERE id = $1"#,
                document_id
            )
            .fetch_optional(&self.pool)
            .await?;
        } else if let Some(chassis_number) = chassis_number {
            document = sqlx::query_as!(
                VehicleDocument,
                r#"SELECT * FROM vehicles_documents WHERE chassis_number = $1"#,
                chassis_number
            )
            .fetch_optional(&self.pool)
            .await?;
        } else if let Some(registration_number) = registration_number {
            document = sqlx::query_as!(
                VehicleDocument,
                r#"SELECT * FROM vehicles_documents WHERE registration_number = $1"#,
                registration_number
            )
            .fetch_optional(&self.pool)
            .await?;
        } else if let Some(plate) = plate {
            document = sqlx::query_as!(
                VehicleDocument,
                r#"SELECT * FROM vehicles_documents WHERE plate = $1"#,
                plate
            )
            .fetch_optional(&self.pool)
            .await?;
        }

        Ok(document)
    }

    async fn list_vehicle_documents(
        &self,
        page: u32,
        limit: usize,
    ) -> Result<Vec<VehicleDocument>, sqlx::Error> {
        let offset = (page - 1) * limit as u32;

        let documents = sqlx::query_as!(
            VehicleDocument,
            r#"SELECT * FROM vehicles_documents LIMIT $1 OFFSET $2"#,
            limit as i64,
            offset as i64
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(documents)
    }

    async fn save_vehicle_document<T: Into<String> + Send>(
        &self,
        params: SaveVehicleDocumentParamsDTO<T>,
    ) -> Result<VehicleDocument, sqlx::Error> {
        let SaveVehicleDocumentParamsDTO {
            chassis_number,
            exercise_year,
            model_year,
            manufacture_year,
            registration_number,
            color,
            make,
            model,
            plate,
            vehicle_id,
        } = params;

        let document = sqlx::query_as!(
            VehicleDocument,
            r#"INSERT INTO vehicles_documents (chassis_number, exercise_year, model_year, manufacture_year, registration_number, color, make, model, plate, vehicle_id) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) RETURNING *"#,
            &chassis_number.into(),
            &exercise_year,
            &model_year,
            &manufacture_year,
            &registration_number.into(),
            &color.into(),
            &make.into(),
            &model.into(),
            &plate.into(),
            Uuid::parse_str(&vehicle_id.unwrap().into()).unwrap(),
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(document)
    }

    async fn delete_vehicle_document(
        &self,
        document_id: Option<Uuid>,
        vehicle_id: Option<Uuid>,
    ) -> Result<Option<VehicleDocument>, sqlx::Error> {
        let mut document = None;

        if let Some(document_id) = document_id {
            document = sqlx::query_as!(
                VehicleDocument,
                r#"DELETE FROM vehicles_documents WHERE id = $1 RETURNING *"#,
                document_id
            )
            .fetch_optional(&self.pool)
            .await?;
        } else if let Some(vehicle_id) = vehicle_id {
            document = sqlx::query_as!(
                VehicleDocument,
                r#"DELETE FROM vehicles_documents WHERE vehicle_id = $1 RETURNING *"#,
                vehicle_id
            )
            .fetch_optional(&self.pool)
            .await?;
        }

        Ok(document)
    }
}
