use actix_web::{web, HttpResponse, Scope};
use sqlx::error::DatabaseError;
use validator::Validate;

use crate::{
    db::route::{RouteExt, RouteStatusExt},
    dtos::{
        request::RequestQueryDTO,
        route::{
            FilterRouteDTO, FilterRouteStatusDTO, RegisterRouteDTO, RegisterRouteStatusDTO,
            RouteListResponseDTO, RouteStatusListResponseDTO,
        },
    },
    error::{ErrorMessage, HttpError},
    AppState,
};

pub fn route_scope() -> Scope {
    web::scope("/api/v1/routes")
        .route("", web::get().to(list_routes))
        .route("", web::post().to(save_route))
        .route("/status", web::post().to(save_route_status))
        .route("/status/{id}", web::get().to(get_route_status))
        .route("/status/{id}", web::delete().to(delete_route_status))
        .route("/status", web::get().to(list_route_status))
        .route("/{id}", web::get().to(get_route))
        .route("/{id}", web::delete().to(delete_route))
        .route("/{id}/status", web::get().to(get_route_status_from_route))
}

pub async fn get_route(
    id: web::Path<uuid::Uuid>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, HttpError> {
    let route = app_state
        .db_client
        .get_route(Some(id.into_inner()))
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    match route {
        Some(route) => Ok(HttpResponse::Ok().json(FilterRouteDTO::filter_route(&route))),
        None => Err(HttpError::from_error_message(ErrorMessage::RouteNotFound)),
    }
}

pub async fn list_routes(
    query: web::Query<RequestQueryDTO>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, HttpError> {
    let query_params: RequestQueryDTO = query.into_inner();

    query_params
        .validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let page = query_params.page.unwrap_or(1);
    let limit = query_params.limit.unwrap_or(50);

    let routes = app_state
        .db_client
        .list_routes(page as u32, limit)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    Ok(HttpResponse::Ok().json(RouteListResponseDTO {
        routes: FilterRouteDTO::filter_routes(&routes),
        results: routes.len(),
    }))
}

pub async fn save_route(
    app_state: web::Data<AppState>,
    body: web::Json<RegisterRouteDTO>,
) -> Result<HttpResponse, HttpError> {
    body.validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let dto = body.into_inner();

    let result = app_state
        .db_client
        .save_route(dto.into_save_route_params_dto())
        .await;

    match result {
        Ok(route) => Ok(HttpResponse::Created().json(FilterRouteDTO::filter_route(&route))),
        Err(sqlx::Error::Database(db_err)) => {
            if db_err.is_foreign_key_violation() {
                return match_foreign_key_violation(&db_err);
            } else {
                Err(HttpError::server_error(db_err.to_string()))
            }
        }
        Err(e) => Err(HttpError::server_error(e.to_string())),
    }
}

pub async fn delete_route(
    id: web::Path<uuid::Uuid>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, HttpError> {
    let route = app_state
        .db_client
        .delete_route(Some(id.into_inner()))
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    match route {
        Some(route) => Ok(HttpResponse::Ok().json(FilterRouteDTO::filter_route(&route))),
        None => Err(HttpError::from_error_message(ErrorMessage::RouteNotFound)),
    }
}

pub async fn get_route_status(
    id: web::Path<uuid::Uuid>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, HttpError> {
    let status = app_state
        .db_client
        .get_route_status(Some(id.into_inner()), None)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    match status {
        Some(status) => {
            Ok(HttpResponse::Ok().json(FilterRouteStatusDTO::filter_route_status(&status)))
        }
        None => Err(HttpError::from_error_message(
            ErrorMessage::RouteStatusNotFound,
        )),
    }
}

pub async fn get_route_status_from_route(
    id: web::Path<uuid::Uuid>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, HttpError> {
    let route = app_state
        .db_client
        .get_route(Some(id.into_inner()))
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let status_id = match route {
        Some(route) => route.status_id,
        None => return Err(HttpError::from_error_message(ErrorMessage::RouteNotFound)),
    };

    let status = app_state
        .db_client
        .get_route_status(Some(status_id), None)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    match status {
        Some(status) => {
            Ok(HttpResponse::Ok().json(FilterRouteStatusDTO::filter_route_status(&status)))
        }
        None => Err(HttpError::from_error_message(
            ErrorMessage::RouteStatusNotFound,
        )),
    }
}

pub async fn list_route_status(
    query: web::Query<RequestQueryDTO>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, HttpError> {
    let query_params: RequestQueryDTO = query.into_inner();

    query_params
        .validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let page = query_params.page.unwrap_or(1);
    let limit = query_params.limit.unwrap_or(50);

    let statuses = app_state
        .db_client
        .list_route_status(page as u32, limit)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    Ok(HttpResponse::Ok().json(RouteStatusListResponseDTO {
        status: FilterRouteStatusDTO::filter_route_statuses(&statuses),
        results: statuses.len(),
    }))
}

pub async fn save_route_status(
    app_state: web::Data<AppState>,
    body: web::Json<RegisterRouteStatusDTO>,
) -> Result<HttpResponse, HttpError> {
    body.validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let result = app_state
        .db_client
        .save_route_status(body.code.as_ref(), &body.description)
        .await;

    match result {
        Ok(status) => {
            Ok(HttpResponse::Created().json(FilterRouteStatusDTO::filter_route_status(&status)))
        }
        Err(sqlx::Error::Database(db_err)) => {
            if db_err.is_unique_violation() {
                Err(HttpError::unique_constraint_violation(
                    ErrorMessage::RouteStatusExist,
                ))
            } else {
                Err(HttpError::server_error(db_err.to_string()))
            }
        }
        Err(e) => Err(HttpError::server_error(e.to_string())),
    }
}

pub async fn delete_route_status(
    id: web::Path<uuid::Uuid>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, HttpError> {
    let status = app_state
        .db_client
        .delete_route_status(Some(id.into_inner()))
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    match status {
        Some(status) => {
            Ok(HttpResponse::Ok().json(FilterRouteStatusDTO::filter_route_status(&status)))
        }
        None => Err(HttpError::from_error_message(
            ErrorMessage::RouteStatusNotFound,
        )),
    }
}

fn match_foreign_key_violation(db_err: &Box<dyn DatabaseError>) -> Result<HttpResponse, HttpError> {
    match db_err.constraint() {
        Some(constraint) => {
            if constraint == "fk_routes_initial_address_id"
                || constraint == "fk_routes_final_address_id"
            {
                Err(HttpError::bad_request(ErrorMessage::AddressNotFound))
            } else if constraint == "fk_routes_vehicle_id" {
                Err(HttpError::bad_request(ErrorMessage::VehicleNotFound))
            } else if constraint == "fk_routes_route_status" {
                Err(HttpError::bad_request(ErrorMessage::RouteStatusNotFound))
            } else {
                Err(HttpError::server_error(db_err.to_string()))
            }
        }
        None => Err(HttpError::server_error(db_err.to_string())),
    }
}
