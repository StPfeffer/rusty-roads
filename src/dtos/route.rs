use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    models::route::{Route, RouteStatus},
    utils::string::is_valid_uuid,
};

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterRouteDTO {
    pub initial_lat: BigDecimal,
    pub initial_long: BigDecimal,
    pub final_lat: Option<BigDecimal>,
    pub final_long: Option<BigDecimal>,

    // #[validate(custom(function = "is_valid_uuid", message = "Driver ID must be a valid UUID"))]
    // pub driver_id: String,
    #[validate(length(
        min = 1,
        max = 20,
        message = "Status must have a maximum of 20 characters"
    ))]
    pub status_id: String,

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

    #[validate(custom(function = "is_valid_uuid", message = "Driver ID must be a valid UUID"))]
    pub driver_id: Option<String>,
}

impl RegisterRouteDTO {
    pub fn into_save_route_params_dto(self) -> SaveRouteParamsDTO<BigDecimal, String> {
        SaveRouteParamsDTO {
            initial_lat: self.initial_lat,
            initial_long: self.initial_long,
            final_lat: self.final_lat,
            final_long: self.final_long,
            driver_id: self.driver_id,
            status_id: self.status_id,
            initial_address_id: self.initial_address_id,
            final_address_id: self.final_address_id,
            vehicle_id: self.vehicle_id,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SaveRouteParamsDTO<B, S> {
    pub initial_lat: B,
    pub initial_long: B,
    pub final_lat: Option<B>,
    pub final_long: Option<B>,
    pub driver_id: Option<S>,
    pub status_id: S,
    pub initial_address_id: Option<S>,
    pub final_address_id: Option<S>,
    pub vehicle_id: S,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilterRouteDTO {
    pub id: String,
    pub started_at: NaiveDateTime,
    pub ended_at: Option<NaiveDateTime>,
    pub total_distance: BigDecimal,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub initial_lat: BigDecimal,
    pub initial_long: BigDecimal,
    pub final_lat: Option<BigDecimal>,
    pub final_long: Option<BigDecimal>,
    pub driver_id: Option<String>,
    pub status_id: String,
    pub initial_address_id: Option<String>,
    pub final_address_id: Option<String>,
    pub vehicle_id: String,
}

impl FilterRouteDTO {
    pub fn filter_route(route: &Route) -> Self {
        FilterRouteDTO {
            id: route.id.to_string(),
            started_at: route.started_at.to_owned(),
            ended_at: route.ended_at.to_owned(),
            total_distance: route.total_distance.to_owned(),
            created_at: route.created_at.to_owned(),
            updated_at: route.updated_at.to_owned(),
            initial_lat: route.initial_lat.to_owned(),
            initial_long: route.initial_long.to_owned(),
            final_lat: if route.final_lat != Some(BigDecimal::from(0)) {
                route.final_lat.to_owned()
            } else {
                None
            },
            final_long: if route.final_long != Some(BigDecimal::from(0)) {
                route.final_long.to_owned()
            } else {
                None
            },
            driver_id: route.driver_id.map(|id| id.to_string()),
            status_id: route.status_id.to_string(),
            initial_address_id: route.initial_address_id.map(|id| id.to_string()),
            final_address_id: route.final_address_id.map(|id| id.to_string()),
            vehicle_id: route.vehicle_id.to_string(),
        }
    }

    pub fn filter_routes(routes: &[Route]) -> Vec<FilterRouteDTO> {
        routes.iter().map(FilterRouteDTO::filter_route).collect()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RouteListResponseDTO {
    pub routes: Vec<FilterRouteDTO>,
    pub results: usize,
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct RegisterRouteStatusDTO {
    pub code: Option<String>,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FilterRouteStatusDTO {
    pub id: String,
    pub code: String,
    pub description: String,
}

impl FilterRouteStatusDTO {
    pub fn filter_route_status(status: &RouteStatus) -> Self {
        FilterRouteStatusDTO {
            id: status.id.to_string(),
            code: status.code.to_owned(),
            description: status.description.to_owned(),
        }
    }

    pub fn filter_route_statuses(statuses: &[RouteStatus]) -> Vec<FilterRouteStatusDTO> {
        statuses
            .iter()
            .map(FilterRouteStatusDTO::filter_route_status)
            .collect()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RouteStatusListResponseDTO {
    pub status: Vec<FilterRouteStatusDTO>,
    pub results: usize,
}
