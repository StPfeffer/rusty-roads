use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, sqlx::FromRow, sqlx::Type, Serialize, Clone)]
pub struct State {
    pub id: Uuid,
    pub name: String,
    pub code: String,
    pub country_id: Uuid,
}
