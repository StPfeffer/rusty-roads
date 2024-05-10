use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, sqlx::FromRow, sqlx::Type, Serialize, Clone)]
pub struct Country {
    pub id: Uuid,
    pub name: String,
    pub alpha_2: String,
    pub alpha_3: String,
    pub numeric_3: String,
}

#[derive(Debug, Deserialize, sqlx::FromRow, sqlx::Type, Serialize, Clone)]
pub struct State {
    pub id: Uuid,
    pub name: String,
    pub code: String,
    pub country_id: Uuid,
}
