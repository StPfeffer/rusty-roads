use actix_web::{web, HttpResponse, Scope};
use validator::Validate;

use crate::{
    db::route::RouteExt,
    dtos::{
        request::RequestQueryDTO,
        route::{FilterRouteDTO, RegisterRouteDTO, RouteListResponseDTO},
    },
    error::{ErrorMessage, HttpError},
    AppState,
};

pub fn route_scope() -> Scope {
    web::scope("/api/v1/routes")
        .route("", web::get().to(list_routes))
        .route("/{id}", web::get().to(get_route))
        .route("", web::post().to(save_route))
        .route("/{id}", web::delete().to(delete_route))
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
