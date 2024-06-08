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
    pub id_cnh_type: Uuid,
    pub id_collaborattor: Uuid,
}
