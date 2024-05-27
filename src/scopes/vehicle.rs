use actix_web::{web, HttpResponse, Scope};
use validator::Validate;

use crate::{
    db::vehicle::VehicleExt,
    dtos::{
        request::RequestQueryDTO,
        vehicle::{FilterVehicleDTO, RegisterVehicleDTO, VehicleListResponseDTO},
    },
    error::{ErrorMessage, HttpError},
    AppState,
};

pub fn vehicle_scope() -> Scope {
    web::scope("/api/v1/vehicles")
        .route("", web::get().to(list_vehicles))
        .route("/{id}", web::get().to(get_vehicle))
        .route("", web::post().to(save_vehicle))
        .route("/{id}", web::delete().to(delete_vehicle))
}

pub async fn get_vehicle(
    id: web::Path<uuid::Uuid>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, HttpError> {
    let vehicle = app_state
        .db_client
        .get_vehicle(Some(id.into_inner()))
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    Ok(HttpResponse::Ok().json(FilterVehicleDTO::filter_vehicle(&vehicle.unwrap())))
}

pub async fn list_vehicles(
    query: web::Query<RequestQueryDTO>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, HttpError> {
    let query_params: RequestQueryDTO = query.into_inner();

    query_params
        .validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let page = query_params.page.unwrap_or(1);
    let limit = query_params.limit.unwrap_or(50);

    let vehicles = app_state
        .db_client
        .list_vehicles(page as u32, limit)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    Ok(HttpResponse::Ok().json(VehicleListResponseDTO {
        vehicles: FilterVehicleDTO::filter_vehicles(&vehicles),
        results: vehicles.len(),
    }))
}

pub async fn save_vehicle(
    app_state: web::Data<AppState>,
    body: web::Json<RegisterVehicleDTO>,
) -> Result<HttpResponse, HttpError> {
    body.validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let result = app_state
        .db_client
        .save_vehicle(&body.name, body.initial_mileage, body.actual_mileage)
        .await;

    match result {
        Ok(state) => Ok(HttpResponse::Created().json(FilterVehicleDTO::filter_vehicle(&state))),
        Err(sqlx::Error::Database(db_err)) => {
            if db_err.is_unique_violation() {
                Err(HttpError::unique_constraint_violation(
                    ErrorMessage::StateExist,
                ))
            } else {
                Err(HttpError::server_error(db_err.to_string()))
            }
        }
        Err(e) => Err(HttpError::server_error(e.to_string())),
    }
}

pub async fn delete_vehicle(
    id: web::Path<uuid::Uuid>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, HttpError> {
    let vehicle = app_state
        .db_client
        .delete_vehicle(Some(id.into_inner()))
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    Ok(HttpResponse::Ok().json(FilterVehicleDTO::filter_vehicle(&vehicle.unwrap())))
}
