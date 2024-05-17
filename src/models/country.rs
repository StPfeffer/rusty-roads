use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a database table storing information about countries.
#[derive(Debug, Deserialize, sqlx::FromRow, sqlx::Type, Serialize, Clone)]
pub struct Country {
    /// The unique identifier of the country.
    pub id: Uuid,
    /// The name of the country.
    pub name: String,
    /// The ISO 3166-1 alpha-2 code of the country.
    pub alpha_2: String,
    /// The ISO 3166-1 alpha-3 code of the country.
    pub alpha_3: String,
    /// The ISO 3166-1 numeric-3 code of the country.
    pub numeric_3: String,
}

/// Represents a database table storing information about states/provinces.
#[derive(Debug, Deserialize, sqlx::FromRow, sqlx::Type, Serialize, Clone)]
pub struct State {
    /// The unique identifier of the state/province.
    pub id: Uuid,
    /// The name of the state/province.
    pub name: String,
    /// The code of the state/province.
    pub code: String,
    /// The foreign key referencing the country to which the state/province belongs.
    pub country_id: Uuid,
}
