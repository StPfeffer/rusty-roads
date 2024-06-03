use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, sqlx::Type, Serialize, Clone)]
pub struct Vehicle {
    pub id: Uuid,
    pub name: String,
    pub initial_mileage: i32,
    pub actual_mileage: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, sqlx::Type, Serialize, Clone)]
pub struct VehicleDocument {
    pub id: Uuid,
    pub chassis_number: String,
    pub exercise_year: i16,
    pub model_year: i16,
    pub manufacture_year: i16,
    pub registration_number: String,
    pub color: String,
    pub make: String,
    pub model: String,
    pub plate: String,
    pub updatd_at: NaiveDateTime,
    pub vehicle_id: Uuid,
}
