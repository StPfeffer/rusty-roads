use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, sqlx::FromRow, sqlx::Type, Serialize, Clone)]
pub struct Country {
    pub id: uuid::Uuid,
    pub name: String,
    pub alpha_2: String,
    pub alpha_3: String,
    pub numeric_3: String,
}
