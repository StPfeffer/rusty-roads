use actix_web::{web, HttpResponse, Scope};
use validator::Validate;

use crate::{
    db::collaborator::CollaboratorExt,
    dtos::{
        collaborator::{
            CollaboratorListResponseDTO, FilterCollaboratorDTO, RegisterCollaboratorDTO,
        },
        request::RequestQueryDTO,
    },
    error::{ErrorMessage, HttpError},
    AppState,
};

pub fn collaborator_scope() -> Scope {
    web::scope("/api/v1/collaborators")
        .route("", web::get().to(list_collaborators))
        .route("/{id}", web::get().to(get_collaborator))
        .route("", web::post().to(save_collaborator))
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

    Ok(
        HttpResponse::Ok().json(FilterCollaboratorDTO::filter_collaborator(
            &collaborator.unwrap(),
        )),
    )
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
                    ErrorMessage::StateExist,
                ))
            } else {
                Err(HttpError::server_error(db_err.to_string()))
            }
        }
        Err(e) => Err(HttpError::server_error(e.to_string())),
    }
}

pub async fn delete_collaborator(
    id: web::Path<uuid::Uuid>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, HttpError> {
    let collaborator = app_state
        .db_client
        .delete_collaborator(Some(id.into_inner()))
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    Ok(
        HttpResponse::Ok().json(FilterCollaboratorDTO::filter_collaborator(
            &collaborator.unwrap(),
        )),
    )
}
