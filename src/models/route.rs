use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct Route {
    pub id: Uuid,
    pub started_at: NaiveDateTime,
    pub ended_at: Option<NaiveDateTime>,
    pub total_distance: BigDecimal,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub initial_lat: BigDecimal,
    pub initial_long: BigDecimal,
    pub final_lat: Option<BigDecimal>,
    pub final_long: Option<BigDecimal>,
    // TODO: Waiting for the driver scope
    // pub driver_id: Uuid,
    pub initial_address_id: Option<Uuid>,
    pub final_address_id: Option<Uuid>,
    pub vehicle_id: Uuid,
}
