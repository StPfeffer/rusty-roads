use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, sqlx::FromRow, sqlx::Type, Serialize, Clone)]
pub struct Driver {
    pub id: Uuid,
    pub cnh_number: String,
    pub cnh_expiration_date: NaiveDate,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub cnh_type_id: Uuid,
    pub collaborator_id: Uuid,
}

#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct CnhType {
    pub id: Uuid,
    pub code: String,
    pub description: String,
}
