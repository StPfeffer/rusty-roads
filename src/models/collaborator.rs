use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, sqlx::FromRow, sqlx::Type, Serialize, Clone)]
pub struct Collaborator {
    pub id: uuid::Uuid,
    pub name: String,
    pub cpf: String,
    pub rg: String,
    pub email: String,
    pub created_at: String,
    pub updated_at: String,
}
