use actix_web::{web, HttpResponse, Scope};
use validator::Validate;

use crate::{
    db::country::CountryExt,
    dtos::{
        country::{CountryListResponseDTO, FilterCountryDTO, RegisterCountryDTO},
        request::RequestQueryDTO,
    },
    error::{ErrorMessage, HttpError},
    AppState,
};

pub fn country_scope() -> Scope {
    web::scope("/api/v1/countries")
        .route("", web::get().to(list_countries))
        .route("/{id}", web::get().to(get_country))
        .route("", web::post().to(save_country))
        .route("/{id}", web::delete().to(delete_country))
}

pub async fn get_country(
    id: web::Path<uuid::Uuid>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, HttpError> {
    let country = app_state
        .db_client
        .get_country(Some(id.into_inner()), None, None, None, None)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    match country {
        Some(country) => Ok(HttpResponse::Ok().json(FilterCountryDTO::filter_country(&country))),
        None => Err(HttpError::from_error_message(ErrorMessage::CountryNotFound)),
    }
}

pub async fn list_countries(
    query: web::Query<RequestQueryDTO>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, HttpError> {
    let query_params: RequestQueryDTO = query.into_inner();

    query_params
        .validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let page = query_params.page.unwrap_or(1);
    let limit = query_params.limit.unwrap_or(10);

    let countries = app_state
        .db_client
        .list_countries(page as u32, limit)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    Ok(HttpResponse::Ok().json(CountryListResponseDTO {
        countries: FilterCountryDTO::filter_countries(&countries),
        results: countries.len(),
    }))
}

pub async fn save_country(
    app_state: web::Data<AppState>,
    body: web::Json<RegisterCountryDTO>,
) -> Result<HttpResponse, HttpError> {
    body.validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let result = app_state
        .db_client
        .save_country(&body.name, &body.alpha_2, &body.alpha_3, &body.numeric_3)
        .await;

    match result {
        Ok(country) => Ok(HttpResponse::Created().json(FilterCountryDTO::filter_country(&country))),
        Err(sqlx::Error::Database(db_err)) => {
            if db_err.is_unique_violation() {
                Err(HttpError::unique_constraint_violation(
                    ErrorMessage::CountryExist,
                ))
            } else {
                Err(HttpError::server_error(db_err.to_string()))
            }
        }
        Err(e) => Err(HttpError::server_error(e.to_string())),
    }
}

pub async fn delete_country(
    id: web::Path<uuid::Uuid>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, HttpError> {
    let country = app_state
        .db_client
        .delete_country(Some(id.into_inner()))
        .await
        .map_err(|_| HttpError::from_error_message(ErrorMessage::ServerError))?;

    match country {
        Some(country) => Ok(HttpResponse::Ok().json(FilterCountryDTO::filter_country(&country))),
        None => Err(HttpError::from_error_message(ErrorMessage::CountryNotFound)),
    }
}
