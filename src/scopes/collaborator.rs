use actix_web::{web, Error, HttpResponse, Scope};
use validator::Validate;

use crate::{
    db::{
        collaborator::CollaboratorExt,
        driver::{CnhTypeExt, DriverExt},
    },
    dtos::{
        collaborator::{
            CollaboratorListResponseDTO, FilterCollaboratorDTO, RegisterCollaboratorDTO,
        },
        driver::{
            CnhTypeListResponseDTO, DriverListResponseDTO, FilterCnhTypeDTO, FilterDriverDTO,
            RegisterDriverDTO,
        },
        request::RequestQueryDTO,
    },
    error::{ErrorMessage, HttpError},
    AppState,
};

pub fn collaborator_scope() -> Scope {
    web::scope("/api/v1/collaborators")
        .route("", web::get().to(list_collaborators))
        .route("/drivers", web::post().to(save_driver))
        .route("/drivers", web::get().to(list_drivers))
        .route("/drivers/cnh", web::get().to(list_cnh_types))
        .route("/drivers/cnh/{id}", web::get().to(get_cnh_type))
        .route("/drivers/{id}", web::get().to(get_driver))
        .route("/drivers/{id}", web::put().to(update_driver))
        .route("/{id}", web::get().to(get_collaborator))
        .route("/{id}/drivers", web::get().to(get_driver_from_collaborator))
        .route(
            "/{id}/drivers",
            web::put().to(update_driver_from_collaborator),
        )
        .route("", web::post().to(save_collaborator))
        .route("/{id}", web::put().to(update_collaborator))
        .route("/{id}", web::delete().to(delete_collaborator))
}

pub async fn get_collaborator(
    id: web::Path<uuid::Uuid>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, HttpError> {
    let collaborator = app_state
        .db_client
        .get_collaborator(Some(id.into_inner()), None, None)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    match collaborator {
        Some(collaborator) => {
            Ok(HttpResponse::Ok().json(FilterCollaboratorDTO::filter_collaborator(&collaborator)))
        }
        None => Err(HttpError::from_error_message(
            ErrorMessage::CollaboratorNotFound,
        )),
    }
}

pub async fn list_collaborators(
    query: web::Query<RequestQueryDTO>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, HttpError> {
    let query_params: RequestQueryDTO = query.into_inner();

    query_params
        .validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let page = query_params.page.unwrap_or(1);
    let limit = query_params.limit.unwrap_or(50);

    let collaborators = app_state
        .db_client
        .list_collaborators(page as u32, limit)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    Ok(HttpResponse::Ok().json(CollaboratorListResponseDTO {
        collaborators: FilterCollaboratorDTO::filter_collaborators(&collaborators),
        results: collaborators.len(),
    }))
}

pub async fn save_collaborator(
    app_state: web::Data<AppState>,
    body: web::Json<RegisterCollaboratorDTO>,
) -> Result<HttpResponse, HttpError> {
    body.validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let result = app_state
        .db_client
        .save_collaborator(&body.name, &body.cpf, &body.rg, &body.email)
        .await;

    match result {
        Ok(collaborator) => {
            Ok(HttpResponse::Created()
                .json(FilterCollaboratorDTO::filter_collaborator(&collaborator)))
        }
        Err(sqlx::Error::Database(db_err)) => {
            if db_err.is_unique_violation() {
                Err(HttpError::unique_constraint_violation(
                    ErrorMessage::CollaboratorExist,
                ))
            } else {
                Err(HttpError::server_error(db_err.to_string()))
            }
        }
        Err(e) => Err(HttpError::server_error(e.to_string())),
    }
}

pub async fn update_collaborator(
    id: web::Path<uuid::Uuid>,
    body: web::Json<RegisterCollaboratorDTO>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, HttpError> {
    body.validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let collaborator_id = Some(id.into_inner());

    let collaborator = app_state
        .db_client
        .get_collaborator(collaborator_id, None, None)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    match collaborator {
        Some(_) => {
            let result = app_state
                .db_client
                .update_collaborator(
                    collaborator_id,
                    &body.name,
                    &body.cpf,
                    &body.rg,
                    &body.email,
                )
                .await;

            return match result {
                Ok(collaborator) => Ok(HttpResponse::Created()
                    .json(FilterCollaboratorDTO::filter_collaborator(&collaborator))),
                Err(sqlx::Error::Database(db_err)) => {
                    if db_err.is_unique_violation() {
                        Err(HttpError::unique_constraint_violation(
                            ErrorMessage::CollaboratorExist,
                        ))
                    } else {
                        Err(HttpError::server_error(db_err.to_string()))
                    }
                }
                Err(e) => Err(HttpError::server_error(e.to_string())),
            };
        }
        None => Err(HttpError::from_error_message(
            ErrorMessage::CollaboratorNotFound,
        )),
    }
}

pub async fn delete_collaborator(
    id: web::Path<uuid::Uuid>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, HttpError> {
    let collaborator_id = Some(id.into_inner());

    app_state
        .db_client
        .delete_driver(None, collaborator_id)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let collaborator = app_state
        .db_client
        .delete_collaborator(collaborator_id)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    match collaborator {
        Some(collaborator) => {
            Ok(HttpResponse::Ok().json(FilterCollaboratorDTO::filter_collaborator(&collaborator)))
        }
        None => Err(HttpError::from_error_message(
            ErrorMessage::CollaboratorNotFound,
        )),
    }
}

pub async fn get_driver(
    id: web::Path<uuid::Uuid>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, HttpError> {
    let driver = app_state
        .db_client
        .get_driver(Some(id.into_inner()), None, None)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    match driver {
        Some(driver) => Ok(HttpResponse::Ok().json(FilterDriverDTO::filter_driver(&driver))),
        None => Err(HttpError::from_error_message(ErrorMessage::DriverNotFound)),
    }
}

pub async fn get_driver_from_collaborator(
    id: web::Path<uuid::Uuid>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, HttpError> {
    let driver = app_state
        .db_client
        .get_driver(None, None, Some(id.into_inner()))
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    match driver {
        Some(driver) => Ok(HttpResponse::Ok().json(FilterDriverDTO::filter_driver(&driver))),
        None => Err(HttpError::from_error_message(ErrorMessage::DriverNotFound)),
    }
}

pub async fn list_drivers(
    query: web::Query<RequestQueryDTO>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, HttpError> {
    let query_params: RequestQueryDTO = query.into_inner();

    query_params
        .validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let page = query_params.page.unwrap_or(1);
    let limit = query_params.limit.unwrap_or(50);

    let drivers = app_state
        .db_client
        .list_drivers(page as u32, limit)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    Ok(HttpResponse::Ok().json(DriverListResponseDTO {
        drivers: FilterDriverDTO::filter_drivers(&drivers),
        results: drivers.len(),
    }))
}

pub async fn save_driver(
    app_state: web::Data<AppState>,
    body: web::Json<RegisterDriverDTO>,
) -> Result<HttpResponse, HttpError> {
    body.validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let result = app_state
        .db_client
        .save_driver(
            &body.cnh_number,
            body.cnh_expiration_date,
            &body.id_cnh_type,
            &body.collaborator_id,
        )
        .await;

    match result {
        Ok(driver) => Ok(HttpResponse::Created().json(FilterDriverDTO::filter_driver(&driver))),
        Err(sqlx::Error::Database(db_err)) => {
            if db_err.is_unique_violation() {
                match db_err.constraint() {
                    Some(constraint) => {
                        if constraint == "fk_drivers_collaborator_id"
                            || constraint == "unq_drivers_cnh_number"
                        {
                            Err(HttpError::from_error_message(ErrorMessage::DriverExist))
                        } else {
                            Err(HttpError::server_error(db_err.to_string()))
                        }
                    }
                    None => Err(HttpError::server_error(db_err.to_string())),
                }
            } else if db_err.is_foreign_key_violation() {
                match db_err.constraint() {
                    Some(constraint) => {
                        if constraint == "fk_drivers_collaborator_id" {
                            Err(HttpError::from_error_message(
                                ErrorMessage::CollaboratorNotFound,
                            ))
                        } else {
                            Err(HttpError::from_error_message(ErrorMessage::CnhTypeNotFound))
                        }
                    }
                    None => Err(HttpError::server_error(db_err.to_string())),
                }
            } else {
                Err(HttpError::server_error(db_err.to_string()))
            }
        }
        Err(e) => Err(HttpError::server_error(e.to_string())),
    }
}

pub async fn update_driver(
    id: web::Path<uuid::Uuid>,
    body: web::Json<RegisterDriverDTO>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, HttpError> {
    body.validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let driver_id = Some(id.into_inner());

    let driver = app_state
        .db_client
        .get_driver(driver_id, None, None)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    match driver {
        Some(_) => {
            let result = app_state
                .db_client
                .update_driver(
                    driver_id,
                    None,
                    &body.cnh_number,
                    body.cnh_expiration_date,
                    &body.id_cnh_type,
                )
                .await;

            return match result {
                Ok(driver) => {
                    Ok(HttpResponse::Accepted().json(FilterDriverDTO::filter_driver(&driver)))
                }
                Err(sqlx::Error::Database(db_err)) => {
                    if db_err.is_unique_violation() {
                        Err(HttpError::unique_constraint_violation(
                            ErrorMessage::DriverExist,
                        ))
                    } else {
                        Err(HttpError::server_error(db_err.to_string()))
                    }
                }
                Err(e) => Err(HttpError::server_error(e.to_string())),
            };
        }
        None => Err(HttpError::from_error_message(ErrorMessage::DriverNotFound)),
    }
}

pub async fn update_driver_from_collaborator(
    id: web::Path<uuid::Uuid>,
    body: web::Json<RegisterDriverDTO>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, HttpError> {
    body.validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let collaborator_id = Some(id.into_inner());

    let driver = app_state
        .db_client
        .get_driver(None, None, collaborator_id)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    match driver {
        Some(_) => {
            let result = app_state
                .db_client
                .update_driver(
                    None,
                    collaborator_id,
                    &body.cnh_number,
                    body.cnh_expiration_date,
                    &body.id_cnh_type,
                )
                .await;

            return match result {
                Ok(driver) => {
                    Ok(HttpResponse::Accepted().json(FilterDriverDTO::filter_driver(&driver)))
                }
                Err(sqlx::Error::Database(db_err)) => {
                    if db_err.is_unique_violation() {
                        Err(HttpError::unique_constraint_violation(
                            ErrorMessage::DriverExist,
                        ))
                    } else {
                        Err(HttpError::server_error(db_err.to_string()))
                    }
                }
                Err(e) => Err(HttpError::server_error(e.to_string())),
            };
        }
        None => Err(HttpError::from_error_message(ErrorMessage::DriverNotFound)),
    }
}

pub async fn get_cnh_type(
    id: web::Path<uuid::Uuid>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, HttpError> {
    let cnh_type = app_state
        .db_client
        .get_cnh_type(Some(id.into_inner()), None)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    match cnh_type {
        Some(cnh_type) => Ok(HttpResponse::Ok().json(FilterCnhTypeDTO::filter_cnh_type(&cnh_type))),
        None => Err(HttpError::from_error_message(ErrorMessage::DriverNotFound)),
    }
}

pub async fn list_cnh_types(
    query: web::Query<RequestQueryDTO>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, HttpError> {
    let query_params: RequestQueryDTO = query.into_inner();

    query_params
        .validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let page = query_params.page.unwrap_or(1);
    let limit = query_params.limit.unwrap_or(50);

    let cnh_types = app_state
        .db_client
        .list_cnh_type(page as u32, limit)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    Ok(HttpResponse::Ok().json(CnhTypeListResponseDTO {
        types: FilterCnhTypeDTO::filter_cnh_types(&cnh_types),
        results: cnh_types.len(),
    }))
}
