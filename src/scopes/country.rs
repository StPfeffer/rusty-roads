use std::str::FromStr;

use actix_web::{body::MessageBody, web, HttpRequest, HttpResponse, Scope};
use validator::Validate;

use crate::{
    db::{country::CountryExt, state::StateExt},
    dtos::{
        country::{CountryListResponseDTO, FilterCountryDTO, RegisterCountryDTO},
        request::RequestQueryDTO,
        state::{FilterStateDTO, StateListResponseDTO},
    },
    error::{ErrorMessage, HttpError},
    utils::string::extract_endpoint_from_path,
    AppState,
};

pub fn country_scope() -> Scope {
    web::scope("/api/v1/countries")
        .route("", web::get().to(list_countries))
        .route("/{id}", web::get().to(get_country))
        .route("/{id}/states", web::get().to(list_country_states))
        .route("", web::post().to(save_country))
        .route("/{id}", web::put().to(update_country))
        .route("/{id}", web::delete().to(delete_country))
        .route("/alpha2/{code}", web::get().to(get_country_by_code))
        .route("/alpha3/{code}", web::get().to(get_country_by_code))
        .route("/numeric3/{code}", web::get().to(get_country_by_code))
        .route(
            "/alpha2/{code}/states",
            web::get().to(list_country_states_by_code),
        )
        .route(
            "/alpha3/{code}/states",
            web::get().to(list_country_states_by_code),
        )
        .route(
            "/numeric3/{code}/states",
            web::get().to(list_country_states_by_code),
        )
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

pub async fn get_country_by_code(
    code: web::Path<String>,
    app_state: web::Data<AppState>,
    request: HttpRequest,
) -> Result<HttpResponse, HttpError> {
    let endpoint = extract_endpoint_from_path(r"/countries/([^/]+)/", &request)?;

    let country = match &endpoint[..] {
        "alpha2" => app_state
            .db_client
            .get_country(None, None, Some(&code), None, None)
            .await
            .map_err(|e| HttpError::server_error(e.to_string()))?,
        "alpha3" => app_state
            .db_client
            .get_country(None, None, None, Some(&code), None)
            .await
            .map_err(|e| HttpError::server_error(e.to_string()))?,
        _ => app_state
            .db_client
            .get_country(None, None, None, None, Some(&code))
            .await
            .map_err(|e| HttpError::server_error(e.to_string()))?,
    };

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
    let limit = query_params.limit.unwrap_or(50);

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

pub async fn update_country(
    id: web::Path<uuid::Uuid>,
    app_state: web::Data<AppState>,
    body: web::Json<RegisterCountryDTO>,
) -> Result<HttpResponse, HttpError> {
    let country = app_state
        .db_client
        .get_country(Some(id.into_inner()), None, None, None, None)
        .await
        .map_err(|_| HttpError::from_error_message(ErrorMessage::ServerError))?;

    match country {
        Some(country) => {
            let country = app_state
                .db_client
                .update_country(
                    country.id,
                    &body.name,
                    &body.alpha_2,
                    &body.alpha_3,
                    &body.numeric_3,
                )
                .await
                .map_err(|_| HttpError::from_error_message(ErrorMessage::ServerError))?;

            Ok(HttpResponse::Ok().json(FilterCountryDTO::filter_country(&country)))
        }
        None => Err(HttpError::from_error_message(ErrorMessage::CountryNotFound)),
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

pub async fn list_country_states(
    id: web::Path<uuid::Uuid>,
    query: web::Query<RequestQueryDTO>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, HttpError> {
    let query_params: RequestQueryDTO = query.into_inner();

    query_params
        .validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let page = query_params.page.unwrap_or(1);
    let limit = query_params.limit.unwrap_or(50);

    let states = app_state
        .db_client
        .list_states_by_country(Some(id.into_inner()), page as u32, limit)
        .await
        .map_err(|_| HttpError::from_error_message(ErrorMessage::ServerError))?;

    Ok(HttpResponse::Ok().json(StateListResponseDTO {
        states: FilterStateDTO::filter_states(&states),
        results: states.len(),
    }))
}

pub async fn list_country_states_by_code(
    code: web::Path<String>,
    query: web::Query<RequestQueryDTO>,
    app_state: web::Data<AppState>,
    request: HttpRequest,
) -> Result<HttpResponse, HttpError> {
    let country_result = get_country_by_code(code, app_state.clone(), request).await;

    match country_result {
        Ok(country_response) => {
            // Use the country_response to get the country id
            let country_id = {
                let body = country_response.into_body();
                let bytes = body.try_into_bytes().unwrap();

                let str_body = std::str::from_utf8(&bytes)
                    .map_err(|_| HttpError::from_error_message(ErrorMessage::ServerError))?;
                let country_dto: FilterCountryDTO = serde_json::from_str(str_body)
                    .map_err(|_| HttpError::from_error_message(ErrorMessage::ServerError))?;
                country_dto.id
            };

            let query_params: RequestQueryDTO = query.into_inner();

            query_params
                .validate()
                .map_err(|e| HttpError::bad_request(e.to_string()))?;

            let page = query_params.page.unwrap_or(1);
            let limit = query_params.limit.unwrap_or(50);

            // Use the country id to get the states
            let states = app_state
                .db_client
                .list_states_by_country(
                    Some(uuid::Uuid::from_str(&country_id).unwrap()),
                    page as u32,
                    limit,
                )
                .await
                .map_err(|_| HttpError::from_error_message(ErrorMessage::ServerError))?;

            Ok(HttpResponse::Ok().json(StateListResponseDTO {
                states: FilterStateDTO::filter_states(&states),
                results: states.len(),
            }))
        }
        Err(e) => Err(e),
    }
}
