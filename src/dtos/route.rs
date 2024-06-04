use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{models::route::Route, utils::uuid::is_valid_uuid};

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterRouteDTO {
    pub total_distance: BigDecimal,
    pub initial_lat: BigDecimal,
    pub initial_long: BigDecimal,
    pub final_lat: BigDecimal,
    pub final_long: BigDecimal,

    // #[validate(custom(function = "is_valid_uuid", message = "Driver ID must be a valid UUID"))]
    // pub driver_id: String,

    #[validate(custom(
        function = "is_valid_uuid",
        message = "Initial address ID must be a valid UUID"
    ))]
    pub initial_address_id: Option<String>,

    #[validate(custom(
        function = "is_valid_uuid",
        message = "Final address ID must be a valid UUID"
    ))]
    pub final_address_id: Option<String>,

    #[validate(custom(
        function = "is_valid_uuid",
        message = "Vehicle ID must be a valid UUID"
    ))]
    pub vehicle_id: String,
}

impl RegisterRouteDTO {
    pub fn into_save_route_params_dto(self) -> SaveRouteParamsDTO<BigDecimal, String> {
        SaveRouteParamsDTO {
            total_distance: self.total_distance,
            initial_lat: self.initial_lat,
            initial_long: self.initial_long,
            final_lat: self.final_lat,
            final_long: self.final_long,
            // driver_id: self.driver_id,
            initial_address_id: self.initial_address_id,
            final_address_id: self.final_address_id,
            vehicle_id: self.vehicle_id,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SaveRouteParamsDTO<B, S> {
    pub total_distance: B,
    pub initial_lat: B,
    pub initial_long: B,
    pub final_lat: B,
    pub final_long: B,
    // pub driver_id: S,
    pub initial_address_id: Option<S>,
    pub final_address_id: Option<S>,
    pub vehicle_id: S,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilterRouteDTO {
    pub total_distance: BigDecimal,
    pub initial_lat: BigDecimal,
    pub initial_long: BigDecimal,
    pub final_lat: BigDecimal,
    pub final_long: BigDecimal,
    // pub driver_id: String,
    pub initial_address_id: Option<String>,
    pub final_address_id: Option<String>,
    pub vehicle_id: String,
}

impl FilterRouteDTO {
    pub fn filter_route(route: &Route) -> Self {
        FilterRouteDTO {
            total_distance: route.total_distance.to_owned(),
            initial_lat: route.initial_lat.to_owned(),
            initial_long: route.initial_long.to_owned(),
            final_lat: route.final_lat.to_owned(),
            final_long: route.final_long.to_owned(),
            // driver_id: route.driver_id.to_string(),
            initial_address_id: Some(route.initial_address_id.unwrap().to_string()),
            final_address_id: Some(route.final_address_id.unwrap().to_string()),
            vehicle_id: route.vehicle_id.to_string(),
        }
    }

    pub fn filter_routes(states: &[Route]) -> Vec<FilterRouteDTO> {
        states.iter().map(FilterRouteDTO::filter_route).collect()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RouteResponseDTO {
    pub status: String,
    pub data: FilterRouteDTO,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RouteListResponseDTO {
    pub routes: Vec<FilterRouteDTO>,
    pub results: usize,
}
