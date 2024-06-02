use actix_web::{web, HttpResponse, Scope};
use validator::Validate;

use crate::{
    db::address::AddressExt,
    dtos::{
        address::{AddressListResponseDTO, FilterAddressDTO, RegisterAddressDTO},
        request::RequestQueryDTO,
    },
    error::{ErrorMessage, HttpError},
    AppState,
};

pub fn address_scope() -> Scope {
    web::scope("/api/v1/addresses")
        .route("", web::get().to(list_addresses))
        .route("/{id}", web::get().to(get_address))
        .route("", web::post().to(save_address))
        .route("/{id}", web::delete().to(delete_address))
}

pub async fn get_address(
    id: web::Path<uuid::Uuid>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, HttpError> {
    let address = app_state
        .db_client
        .get_address(Some(id.into_inner()), None, None)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    match address {
        Some(address) => Ok(HttpResponse::Ok().json(FilterAddressDTO::filter_address(&address))),
        None => Err(HttpError::from_error_message(ErrorMessage::AddressNotFound)),
    }
}

pub async fn list_addresses(
    query: web::Query<RequestQueryDTO>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, HttpError> {
    let query_params: RequestQueryDTO = query.into_inner();

    query_params
        .validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let page = query_params.page.unwrap_or(1);
    let limit = query_params.limit.unwrap_or(50);

    let addresses = app_state
        .db_client
        .list_addresses(page as u32, limit)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    Ok(HttpResponse::Ok().json(AddressListResponseDTO {
        addresses: FilterAddressDTO::filter_addresses(&addresses),
        results: addresses.len(),
    }))
}

pub async fn save_address(
    app_state: web::Data<AppState>,
    body: web::Json<RegisterAddressDTO>,
) -> Result<HttpResponse, HttpError> {
    body.validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let dto = body.into_inner();

    let result = app_state
        .db_client
        .save_address(dto.into_save_address_params_dto())
        .await;

    match result {
        Ok(address) => Ok(HttpResponse::Created().json(FilterAddressDTO::filter_address(&address))),
        Err(sqlx::Error::Database(db_err)) => {
            if db_err.is_unique_violation() {
                Err(HttpError::unique_constraint_violation(
                    ErrorMessage::AddressExist,
                ))
            } else if db_err.is_foreign_key_violation() {
                Err(HttpError::bad_request(ErrorMessage::CityNotFound))
            } else {
                Err(HttpError::server_error(db_err.to_string()))
            }
        }
        Err(e) => Err(HttpError::server_error(e.to_string())),
    }
}

pub async fn delete_address(
    id: web::Path<uuid::Uuid>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, HttpError> {
    let address = app_state
        .db_client
        .delete_address(Some(id.into_inner()))
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    match address {
        Some(address) => Ok(HttpResponse::Ok().json(FilterAddressDTO::filter_address(&address))),
        None => Err(HttpError::from_error_message(ErrorMessage::AddressNotFound)),
    }
}
