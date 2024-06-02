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
