use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    models::driver::{CnhType, Driver},
    utils::string::is_valid_uuid,
};

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
    pub collaborator_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilterDriverDTO {
    pub id: String,
    pub cnh_number: String,
    pub cnh_expiration_date: NaiveDate,
    pub cnh_type_id: String,
    pub collaborator_id: String,
}

impl FilterDriverDTO {
    pub fn filter_driver(driver: &Driver) -> Self {
        FilterDriverDTO {
            id: driver.id.to_string(),
            cnh_number: driver.cnh_number.to_owned(),
            cnh_expiration_date: driver.cnh_expiration_date.to_owned(),
            cnh_type_id: driver.cnh_type_id.to_string(),
            collaborator_id: driver.collaborator_id.to_string(),
        }
    }

    pub fn filter_drivers(drivers: &[Driver]) -> Vec<FilterDriverDTO> {
        drivers.iter().map(FilterDriverDTO::filter_driver).collect()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DriverListResponseDTO {
    pub drivers: Vec<FilterDriverDTO>,
    pub results: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FilterCnhTypeDTO {
    pub id: String,
    pub code: String,
    pub description: String,
}

impl FilterCnhTypeDTO {
    pub fn filter_cnh_type(cnh_type: &CnhType) -> Self {
        FilterCnhTypeDTO {
            id: cnh_type.id.to_string(),
            code: cnh_type.code.to_owned(),
            description: cnh_type.description.to_owned(),
        }
    }

    pub fn filter_cnh_types(cnh_types: &[CnhType]) -> Vec<FilterCnhTypeDTO> {
        cnh_types
            .iter()
            .map(FilterCnhTypeDTO::filter_cnh_type)
            .collect()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CnhTypeListResponseDTO {
    pub types: Vec<FilterCnhTypeDTO>,
    pub results: usize,
}
