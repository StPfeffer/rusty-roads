use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct Address {
    pub id: Uuid,
    pub address: String,
    pub number: String,
    pub neighbourhood: String,
    pub reference: Option<String>,
    pub complement: Option<String>,
    pub zip_code: String,
    pub latitude: Option<BigDecimal>,
    pub longitude: Option<BigDecimal>,
    pub city_id: Uuid,
}
