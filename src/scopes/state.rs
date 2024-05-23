use actix_web::{web, HttpResponse, Scope};
use validator::Validate;

use crate::{
    db::state::StateExt,
    dtos::{
        request::RequestQueryDTO,
        state::{FilterStateDTO, RegisterStateDTO, StateListResponseDTO},
    },
    error::{ErrorMessage, HttpError},
    AppState,
};

pub fn state_scope() -> Scope {
    web::scope("/api/v1/states")
        .route("", web::get().to(list_states))
        .route("/{id}", web::get().to(get_state))
        .route("", web::post().to(save_state))
        .route("/{id}", web::delete().to(delete_state))
}

pub async fn get_state(
    id: web::Path<uuid::Uuid>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, HttpError> {
    let state = app_state
        .db_client
        .get_state(Some(id.into_inner()), None, None)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    Ok(HttpResponse::Ok().json(FilterStateDTO::filter_state(&state.unwrap())))
}

pub async fn list_states(
    query: web::Query<RequestQueryDTO>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, HttpError> {
    let query_params: RequestQueryDTO = query.into_inner();

    query_params
        .validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let page = query_params.page.unwrap_or(1);
    let limit = query_params.limit.unwrap_or(10);

    let states = app_state
        .db_client
        .list_states(page as u32, limit)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    Ok(HttpResponse::Ok().json(StateListResponseDTO {
        states: FilterStateDTO::filter_states(&states),
        results: states.len(),
    }))
}

pub async fn save_state(
    app_state: web::Data<AppState>,
    body: web::Json<RegisterStateDTO>,
) -> Result<HttpResponse, HttpError> {
    body.validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let result = app_state
        .db_client
        .save_state(&body.name, &body.code, &body.country_id)
        .await;

    match result {
        Ok(state) => Ok(HttpResponse::Created().json(FilterStateDTO::filter_state(&state))),
        Err(sqlx::Error::Database(db_err)) => {
            if db_err.is_unique_violation() {
                Err(HttpError::unique_constraint_violation(
                    ErrorMessage::StateExist,
                ))
            } else if db_err.is_foreign_key_violation() {
                Err(HttpError::bad_request(ErrorMessage::CountryNotFound))
            } else {
                Err(HttpError::server_error(db_err.to_string()))
            }
        }
        Err(e) => Err(HttpError::server_error(e.to_string())),
    }
}

pub async fn delete_state(
    id: web::Path<uuid::Uuid>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, HttpError> {
    let state = app_state
        .db_client
        .delete_state(Some(id.into_inner()))
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    Ok(HttpResponse::Ok().json(FilterStateDTO::filter_state(&state.unwrap())))
}
