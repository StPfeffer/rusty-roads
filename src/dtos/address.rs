use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{models::address::Address, utils::utils::is_valid_uuid};

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterAddressDTO {
    #[validate(length(
        min = 1,
        max = 100,
        message = "Address must have a maximum of 100 characters"
    ))]
    pub address: String,

    #[validate(length(
        min = 1,
        max = 10,
        message = "Number must have a maximum of 10 characters"
    ))]
    pub number: String,

    #[validate(length(
        min = 1,
        max = 60,
        message = "Neighbourhood must have a maximum of 60 characters"
    ))]
    pub neighbourhood: String,

    #[validate(length(
        min = 1,
        max = 60,
        message = "Reference must have a maximum of 60 characters"
    ))]
    pub reference: Option<String>,

    #[validate(length(
        min = 1,
        max = 60,
        message = "Complement must have a maximum of 60 characters"
    ))]
    pub complement: Option<String>,

    #[validate(length(
        min = 5,
        max = 8,
        message = "Zip code must have a minimum of 5 and a maximum of 8 characters"
    ))]
    pub zip_code: String,

    pub latitude: Option<BigDecimal>,
    pub longitude: Option<BigDecimal>,

    #[validate(custom(function = "is_valid_uuid", message = "City ID must be a valid UUID"))]
    pub city_id: String,
}

impl RegisterAddressDTO {
    pub fn into_save_address_params_dto(self) -> SaveAddressParamsDTO<String, BigDecimal> {
        SaveAddressParamsDTO {
            address: self.address,
            number: self.number,
            neighbourhood: self.neighbourhood,
            reference: self.reference,
            complement: self.complement,
            zip_code: self.zip_code,
            latitude: self.latitude,
            longitude: self.longitude,
            city_id: self.city_id,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SaveAddressParamsDTO<T, B> {
    pub address: T,
    pub number: T,
    pub neighbourhood: T,
    pub reference: Option<T>,
    pub complement: Option<T>,
    pub zip_code: T,
    pub latitude: Option<B>,
    pub longitude: Option<B>,
    pub city_id: T,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilterAddressDTO {
    pub id: String,
    pub address: String,
    pub number: String,
    pub neighbourhood: String,
    pub reference: Option<String>,
    pub complement: Option<String>,
    pub zip_code: String,
    pub latitude: Option<BigDecimal>,
    pub longitude: Option<BigDecimal>,
    pub city_id: String,
}

impl FilterAddressDTO {
    pub fn filter_address(address: &Address) -> Self {
        FilterAddressDTO {
            id: address.id.to_string(),
            address: address.address.to_owned(),
            number: address.number.to_owned(),
            neighbourhood: address.neighbourhood.to_owned(),
            reference: address.reference.to_owned(),
            complement: address.complement.to_owned(),
            zip_code: address.zip_code.to_owned(),
            latitude: address.latitude.to_owned(),
            longitude: address.longitude.to_owned(),
            city_id: address.city_id.to_string(),
        }
    }

    pub fn filter_addresses(addresses: &[Address]) -> Vec<FilterAddressDTO> {
        addresses
            .iter()
            .map(FilterAddressDTO::filter_address)
            .collect()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddressListResponseDTO {
    pub addresses: Vec<FilterAddressDTO>,
    pub results: usize,
}
