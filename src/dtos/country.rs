use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::models::country::Country;

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct RegisterCountryDTO {
    #[validate(length(
        min = 1,
        max = 100,
        message = "Name must have a maximum of 100 characters"
    ))]
    pub name: String,

    #[validate(length(min = 2, max = 2, message = "Alpha 2 code must be 2 characters long"))]
    #[serde(rename = "alpha2")]
    pub alpha_2: String,

    #[validate(length(min = 3, max = 3, message = "Alpha 3 code must be 3 characters long"))]
    #[serde(rename = "alpha3")]
    pub alpha_3: String,

    #[validate(length(min = 3, max = 3, message = "Numeric 3 code must be 3 characters long"))]
    #[serde(rename = "numeric3")]
    pub numeric_3: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FilterCountryDTO {
    pub id: String,
    pub name: String,

    #[serde(rename = "alpha2")]
    pub alpha_2: String,

    #[serde(rename = "alpha3")]
    pub alpha_3: String,

    #[serde(rename = "numeric3")]
    pub numeric_3: String,
}

impl FilterCountryDTO {
    pub fn filter_country(country: &Country) -> Self {
        FilterCountryDTO {
            id: country.id.to_string(),
            name: country.name.to_owned(),
            alpha_2: country.alpha_2.to_owned(),
            alpha_3: country.alpha_3.to_owned(),
            numeric_3: country.numeric_3.to_owned(),
        }
    }

    pub fn filter_countries(countries: &[Country]) -> Vec<FilterCountryDTO> {
        countries
            .iter()
            .map(FilterCountryDTO::filter_country)
            .collect()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CountryListResponseDTO {
    pub countries: Vec<FilterCountryDTO>,
    pub results: usize,
}
