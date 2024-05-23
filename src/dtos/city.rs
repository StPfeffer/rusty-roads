use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{models::city::City, utils::uuid::is_valid_uuid};

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct RegisterCityDTO {
    #[validate(length(
        min = 1,
        max = 100,
        message = "City name must have a maximum of 100 characters"
    ))]
    pub name: String,

    #[validate(length(min = 7, max = 7, message = "City code must be 7 characters long."))]
    pub code: String,

    #[validate(custom(function = "is_valid_uuid", message = "State ID must be a valid UUID"))]
    #[serde(rename = "stateId")]
    pub state_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FilterCityDTO {
    pub id: String,
    pub name: String,
    pub code: String,
    #[serde(rename = "stateId")]
    pub state_id: String,
}

impl FilterCityDTO {
    pub fn filter_city(city: &City) -> Self {
        FilterCityDTO {
            id: city.id.to_string(),
            name: city.name.to_owned(),
            code: city.code.to_owned(),
            state_id: city.state_id.to_string(),
        }
    }

    pub fn filter_cities(cities: &[City]) -> Vec<FilterCityDTO> {
        cities.iter().map(FilterCityDTO::filter_city).collect()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CityResponseDTO {
    pub status: String,
    pub data: FilterCityDTO,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CityListResponseDTO {
    pub cities: Vec<FilterCityDTO>,
    pub results: usize,
}
