use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{models::driver::Driver, utils::uuid::is_valid_uuid};

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterDriverDTO {
    #[validate(length(min = 11, max = 11, message = "CNH number must be 11 characters long"))]
    pub cnh_number: String,

    pub cnh_expiration_date: NaiveDate,

    #[validate(custom(
        function = "is_valid_uuid",
        message = "CNH Type ID must be a valid UUID"
    ))]
    pub id_cnh_type: String,

    #[validate(custom(
        function = "is_valid_uuid",
        message = "Collaborator ID must be a valid UUID"
    ))]
    pub id_collaborattor: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FilterDriverDTO {
    pub id: String,
    #[serde(rename = "cnhNumber")]
    pub cnh_number: String,
    pub collaborator_id: String,
}

impl FilterDriverDTO {
    pub fn filter_driver(driver: &Driver) -> Self {
        FilterDriverDTO {
            id: driver.id.to_string(),
            cnh_number: driver.cnh_number.to_owned(),
            collaborator_id: driver.id_collaborattor.to_string(),
        }
    }

    pub fn filter_drivers(drivers: &[Driver]) -> Vec<FilterDriverDTO> {
        drivers.iter().map(FilterDriverDTO::filter_driver).collect()
    }
}

pub struct DriverResponseDTO {
    pub status: String,
    pub data: FilterDriverDTO,
}

pub struct DriverListResponseDTO {
    pub drivers: Vec<FilterDriverDTO>,
    pub results: usize,
}
