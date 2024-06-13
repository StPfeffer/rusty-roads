use actix_web::{web, HttpResponse, Scope};
use validator::Validate;

use crate::{
    db::vehicle::{VehicleDocumentExt, VehicleExt},
    dtos::{
        request::RequestQueryDTO,
        vehicle::{
            FilterVehicleDTO, FilterVehicleDocumentDTO, RegisterVehicleDTO,
            RegisterVehicleDocumentDTO, VehicleDocumentListResponseDTO, VehicleListResponseDTO,
        },
    },
    error::{ErrorMessage, HttpError},
    AppState,
};

pub fn vehicle_scope() -> Scope {
    web::scope("/api/v1/vehicles")
        .route("", web::get().to(list_vehicles))
        .route("/documents", web::get().to(list_vehicles_documents))
        .route("/documents/{id}", web::get().to(get_vehicle_document))
        .route("/documents/{id}", web::put().to(update_vehicle_document))
        .route("/documents/{id}", web::delete().to(delete_vehicle_document))
        .route("/{id}", web::get().to(get_vehicle))
        .route("/{id}", web::put().to(update_vehicle))
        .route("", web::post().to(save_vehicle))
        .route("/{id}", web::delete().to(delete_vehicle))
        .route(
            "/{id}/documents",
            web::get().to(get_vehicle_document_from_vehicle),
        )
        .route(
            "/{id}/documents",
            web::post().to(save_vehicle_document_from_vehicle),
        )
        .route(
            "/{id}/documents",
            web::delete().to(delete_vehicle_document_from_vehicle),
        )
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

    match vehicle {
        Some(vehicle) => Ok(HttpResponse::Ok().json(FilterVehicleDTO::filter_vehicle(&vehicle))),
        None => Err(HttpError::from_error_message(ErrorMessage::VehicleNotFound)),
    }
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

    let mut actual_mileage = body.initial_mileage;
    if let Some(body_actual_mileage) = body.actual_mileage {
        actual_mileage = body_actual_mileage
    }

    let result = app_state
        .db_client
        .save_vehicle(&body.name, body.initial_mileage, actual_mileage)
        .await;

    match result {
        Ok(state) => Ok(HttpResponse::Created().json(FilterVehicleDTO::filter_vehicle(&state))),
        Err(sqlx::Error::Database(db_err)) => {
            if db_err.is_unique_violation() {
                Err(HttpError::unique_constraint_violation(
                    ErrorMessage::VehicleExist,
                ))
            } else {
                Err(HttpError::server_error(db_err.to_string()))
            }
        }
        Err(e) => Err(HttpError::server_error(e.to_string())),
    }
}

pub async fn update_vehicle(
    id: web::Path<uuid::Uuid>,
    body: web::Json<RegisterVehicleDTO>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, HttpError> {
    body.validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let vehicle_id = Some(id.into_inner());

    let vehicle = app_state
        .db_client
        .get_vehicle(vehicle_id)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    match vehicle {
        Some(_) => {
            let result = app_state
                .db_client
                .update_vehicle(
                    vehicle_id,
                    &body.name,
                    Some(body.initial_mileage),
                    body.actual_mileage.unwrap(),
                )
                .await;

            return match result {
                Ok(vehicle) => {
                    Ok(HttpResponse::Created().json(FilterVehicleDTO::filter_vehicle(&vehicle)))
                }
                Err(sqlx::Error::Database(db_err)) => {
                    if db_err.is_unique_violation() {
                        Err(HttpError::unique_constraint_violation(
                            ErrorMessage::VehicleExist,
                        ))
                    } else {
                        Err(HttpError::server_error(db_err.to_string()))
                    }
                }
                Err(e) => Err(HttpError::server_error(e.to_string())),
            };
        }
        None => Err(HttpError::from_error_message(ErrorMessage::VehicleNotFound)),
    }
}

pub async fn delete_vehicle(
    id: web::Path<uuid::Uuid>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, HttpError> {
    let vehicle_id = Some(id.into_inner());

    app_state
        .db_client
        .delete_vehicle_document(None, vehicle_id)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let vehicle = app_state
        .db_client
        .delete_vehicle(vehicle_id)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    match vehicle {
        Some(vehicle) => Ok(HttpResponse::Ok().json(FilterVehicleDTO::filter_vehicle(&vehicle))),
        None => Err(HttpError::from_error_message(ErrorMessage::VehicleNotFound)),
    }
}

pub async fn get_vehicle_document(
    id: web::Path<uuid::Uuid>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, HttpError> {
    let document = app_state
        .db_client
        .get_vehicle_document(Some(id.into_inner()), None, None, None, None)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    match document {
        Some(document) => {
            Ok(HttpResponse::Ok().json(FilterVehicleDocumentDTO::filter_document(&document)))
        }
        None => Err(HttpError::from_error_message(
            ErrorMessage::VehicleDocumentNotFound,
        )),
    }
}

pub async fn get_vehicle_document_from_vehicle(
    id: web::Path<uuid::Uuid>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, HttpError> {
    let document = app_state
        .db_client
        .get_vehicle_document(None, Some(id.into_inner()), None, None, None)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    match document {
        Some(document) => {
            Ok(HttpResponse::Ok().json(FilterVehicleDocumentDTO::filter_document(&document)))
        }
        None => Err(HttpError::from_error_message(
            ErrorMessage::VehicleDocumentNotFound,
        )),
    }
}

pub async fn list_vehicles_documents(
    query: web::Query<RequestQueryDTO>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, HttpError> {
    let query_params: RequestQueryDTO = query.into_inner();

    query_params
        .validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let page = query_params.page.unwrap_or(1);
    let limit = query_params.limit.unwrap_or(50);

    let documents = app_state
        .db_client
        .list_vehicle_documents(page as u32, limit)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    Ok(HttpResponse::Ok().json(VehicleDocumentListResponseDTO {
        documents: FilterVehicleDocumentDTO::filter_documents(&documents),
        results: documents.len(),
    }))
}

pub async fn save_vehicle_document_from_vehicle(
    id: web::Path<uuid::Uuid>,
    app_state: web::Data<AppState>,
    body: web::Json<RegisterVehicleDocumentDTO>,
) -> Result<HttpResponse, HttpError> {
    body.validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let mut dto = body.into_inner();
    dto.vehicle_id = Some(id.to_string()); // Uses the vehicle ID from path

    let result = app_state
        .db_client
        .save_vehicle_document(dto.into_save_vehicle_document_params_dto())
        .await;

    match result {
        Ok(state) => {
            Ok(HttpResponse::Created().json(FilterVehicleDocumentDTO::filter_document(&state)))
        }
        Err(sqlx::Error::Database(db_err)) => {
            if db_err.is_unique_violation() {
                Err(HttpError::unique_constraint_violation(
                    ErrorMessage::VehicleDocumentExist,
                ))
            } else if db_err.is_foreign_key_violation() {
                Err(HttpError::bad_request(ErrorMessage::VehicleNotFound))
            } else {
                Err(HttpError::server_error(db_err.to_string()))
            }
        }
        Err(e) => Err(HttpError::server_error(e.to_string())),
    }
}

pub async fn update_vehicle_document(
    id: web::Path<uuid::Uuid>,
    body: web::Json<RegisterVehicleDocumentDTO>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, HttpError> {
    body.validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let document_id = Some(id.into_inner());

    let dto = body.into_inner();

    let document = app_state
        .db_client
        .get_vehicle_document(document_id, None, None, None, None)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    match document {
        Some(_) => {
            let result = app_state
                .db_client
                .update_vehicle_document(document_id, dto.into_save_vehicle_document_params_dto())
                .await;

            return match result {
                Ok(document) => Ok(HttpResponse::Created()
                    .json(FilterVehicleDocumentDTO::filter_document(&document))),
                Err(sqlx::Error::Database(db_err)) => {
                    if db_err.is_unique_violation() {
                        Err(HttpError::unique_constraint_violation(
                            ErrorMessage::VehicleDocumentExist,
                        ))
                    } else {
                        Err(HttpError::server_error(db_err.to_string()))
                    }
                }
                Err(e) => Err(HttpError::server_error(e.to_string())),
            };
        }
        None => Err(HttpError::from_error_message(
            ErrorMessage::VehicleDocumentNotFound,
        )),
    }
}

pub async fn delete_vehicle_document(
    id: web::Path<uuid::Uuid>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, HttpError> {
    let document = app_state
        .db_client
        .delete_vehicle_document(Some(id.into_inner()), None)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    match document {
        Some(document) => {
            Ok(HttpResponse::Ok().json(FilterVehicleDocumentDTO::filter_document(&document)))
        }
        None => Err(HttpError::from_error_message(
            ErrorMessage::VehicleDocumentNotFound,
        )),
    }
}

pub async fn delete_vehicle_document_from_vehicle(
    id: web::Path<uuid::Uuid>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, HttpError> {
    let document = app_state
        .db_client
        .delete_vehicle_document(None, Some(id.into_inner()))
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    match document {
        Some(document) => {
            Ok(HttpResponse::Ok().json(FilterVehicleDocumentDTO::filter_document(&document)))
        }
        None => Err(HttpError::from_error_message(
            ErrorMessage::VehicleDocumentNotFound,
        )),
    }
}
