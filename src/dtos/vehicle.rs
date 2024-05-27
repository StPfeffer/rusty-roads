use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::models::vehicle::Vehicle;

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterVehicleDTO {
    #[validate(length(
        min = 1,
        max = 50,
        message = "Name must have a maximum of 50 characters"
    ))]
    pub name: String,

    pub initial_mileage: i32,
    pub actual_mileage: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilterVehicleDTO {
    pub id: String,
    pub name: String,
    pub initial_mileage: i32,
    pub actual_mileage: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl FilterVehicleDTO {
    pub fn filter_vehicle(vehicle: &Vehicle) -> Self {
        FilterVehicleDTO {
            id: vehicle.id.to_string(),
            name: vehicle.name.to_owned(),
            initial_mileage: vehicle.initial_mileage.to_owned(),
            actual_mileage: vehicle.actual_mileage.to_owned(),
            created_at: vehicle.created_at.to_owned(),
            updated_at: vehicle.updated_at.to_owned(),
        }
    }

    pub fn filter_vehicles(states: &[Vehicle]) -> Vec<FilterVehicleDTO> {
        states
            .iter()
            .map(FilterVehicleDTO::filter_vehicle)
            .collect()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VehicleResponseDTO {
    pub status: String,
    pub data: FilterVehicleDTO,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VehicleListResponseDTO {
    pub vehicles: Vec<FilterVehicleDTO>,
    pub results: usize,
}
