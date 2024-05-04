use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::models::Country;

#[derive(Serialize, Deserialize, Validate)]
pub struct RequestQueryDTO {
    #[validate(range(min = 1))]
    pub page: Option<usize>,

    #[validate(range(min = 1, max = 50))]
    pub limit: Option<usize>,
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
    pub capital: String,
}

impl FilterCountryDTO {
    pub fn filter_country(country: &Country) -> Self {
        FilterCountryDTO {
            id: country.id.to_string(),
            name: country.name.to_owned(),
            alpha_2: country.alpha_2.to_owned(),
            alpha_3: country.alpha_3.to_owned(),
            numeric_3: country.numeric_3.to_owned(),
            capital: country.capital.to_owned(),
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
pub struct CountryData {
    pub country: FilterCountryDTO,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CountryResponseDTO {
    pub status: String,
    pub data: CountryData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CountryListResponseDTO {
    pub status: String,
    pub countries: Vec<FilterCountryDTO>,
    pub results: usize,
}
