use actix_web::{web, HttpResponse, Scope};
use validator::Validate;

use crate::{
    db::city::CityExt,
    dtos::{
        city::{CityListResponseDTO, FilterCityDTO, RegisterCityDTO},
        request::RequestQueryDTO,
    },
    error::{ErrorMessage, HttpError},
    AppState,
};

pub fn city_scope() -> Scope {
    web::scope("/api/v1/cities")
        .route("", web::get().to(list_cities))
        .route("/{id}", web::get().to(get_city))
        .route("", web::post().to(save_city))
        .route("/{id}", web::delete().to(delete_city))
}

pub async fn get_city(
    id: web::Path<uuid::Uuid>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, HttpError> {
    let city = app_state
        .db_client
        .get_city(Some(id.into_inner()), None)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    match city {
        Some(city) => Ok(HttpResponse::Ok().json(FilterCityDTO::filter_city(&city))),
        None => Err(HttpError::from_error_message(ErrorMessage::CityNotFound)),
    }
}

pub async fn list_cities(
    query: web::Query<RequestQueryDTO>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, HttpError> {
    let query_params: RequestQueryDTO = query.into_inner();

    query_params
        .validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let page = query_params.page.unwrap_or(1);
    let limit = query_params.limit.unwrap_or(10);

    let cities = app_state
        .db_client
        .list_cities(page as u32, limit)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    Ok(HttpResponse::Ok().json(CityListResponseDTO {
        cities: FilterCityDTO::filter_cities(&cities),
        results: cities.len(),
    }))
}

pub async fn save_city(
    app_state: web::Data<AppState>,
    body: web::Json<RegisterCityDTO>,
) -> Result<HttpResponse, HttpError> {
    body.validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let result = app_state
        .db_client
        .save_city(&body.name, &body.code, &body.state_id)
        .await;

    match result {
        Ok(city) => Ok(HttpResponse::Created().json(FilterCityDTO::filter_city(&city))),
        Err(sqlx::Error::Database(db_err)) => {
            if db_err.is_unique_violation() {
                Err(HttpError::unique_constraint_violation(
                    ErrorMessage::StateExist,
                ))
            } else if db_err.is_foreign_key_violation() {
                Err(HttpError::bad_request(ErrorMessage::StateNotFound))
            } else {
                Err(HttpError::server_error(db_err.to_string()))
            }
        }
        Err(e) => Err(HttpError::server_error(e.to_string())),
    }
}

pub async fn delete_city(
    id: web::Path<uuid::Uuid>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, HttpError> {
    let city = app_state
        .db_client
        .delete_city(Some(id.into_inner()))
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    match city {
        Some(city) => Ok(HttpResponse::Ok().json(FilterCityDTO::filter_city(&city))),
        None => Err(HttpError::from_error_message(ErrorMessage::CityNotFound)),
    }
}
