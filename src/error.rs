use std::fmt::{self};

use actix_web::{HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
    }
}

#[derive(Serialize, Deserialize)]
pub struct ResponseDetails {
    pub status: String,
    pub code: String,
    pub message: String,
    pub hint: String,
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub error: ResponseDetails,
}

#[derive(Debug, PartialEq)]
pub enum ErrorMessage {
    ServerError,
    CountryExist,
    CountryNotFound,
    StateExist,
    StateNotFound,
    CityExist,
    CityNotFound,
    AddressExist,
    AddressNotFound,
    CollaboratorExist,
    CollaboratorNotFound,
    VehicleExist,
    VehicleNotFound,
    VehicleDocumentExist,
    VehicleDocumentNotFound,
    RouteNotFound,
}

impl fmt::Display for ErrorMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

impl From<ErrorMessage> for String {
    fn from(error_message: ErrorMessage) -> Self {
        error_message.to_string()
    }
}

impl ErrorMessage {
    fn to_str(&self) -> &str {
        match self {
            ErrorMessage::ServerError => "Server Error. Please try again later",
            ErrorMessage::CountryExist => "There is already a country with the provided data",
            ErrorMessage::CountryNotFound => "The country with the provided ID does not exist in our records",
            ErrorMessage::StateExist => "There is already a state with the provided code and countryId",
            ErrorMessage::StateNotFound => "The state with the provided ID does not exist in our records",
            ErrorMessage::CityExist => "There is already a city with the provided code",
            ErrorMessage::CityNotFound => "The city with the provided ID does not exist in our records",
            ErrorMessage::AddressExist => "There is already an address with the provided address, number and zipCode"   ,
            ErrorMessage::AddressNotFound => "The address with the provided ID does not exist in our records",
            ErrorMessage::CollaboratorExist => "There is already a collaborator with the provided email or cpf",
            ErrorMessage::CollaboratorNotFound => "The collaborator with the provided ID, email or cpf does not exist in our ,records",
            ErrorMessage::VehicleExist => "There is already a vehicle with the provided data",
            ErrorMessage::VehicleNotFound => "The vehicle with the provided ID does not exist in our records",
            ErrorMessage::VehicleDocumentExist => "There is already a document for the vehicle with the provided chassisNumber, ,registrationNumber or plate",
            ErrorMessage::VehicleDocumentNotFound => "The document for the vehicle with the provided ID does not exist in our ,records",
            ErrorMessage::RouteNotFound => "There route with the provided ID does not exist in our records",
        }
    }

    fn hint(&self) -> &str {
        match self {
            ErrorMessage::ServerError => "Check server logs for more details and ensure the server is running correctly.",
            ErrorMessage::CountryExist => "Verify the country data you are trying to add is unique and does not already exist.",
            ErrorMessage::CountryNotFound => "Ensure the countryId is correct and exists in the database. Use the 'GET /api/v1/countries' endpoint to retrieve available country IDs.",
            ErrorMessage::StateExist => "Verify the state code and countryId are unique and do not already exist.",
            ErrorMessage::StateNotFound => "Ensure the stateId is correct and exists in the database. Use the 'GET /api/v1/states' endpoint to retrieve available state IDs.",
            ErrorMessage::CityExist => "Verify the city code is unique and does not already exist.",
            ErrorMessage::CityNotFound => "Ensure the cityId is correct and exists in the database. Use the 'GET /api/v1/cities' endpoint to retrieve available city IDs.",
            ErrorMessage::AddressExist => "Verify the address details (address, number and zipCode) are unique and do not already exist.",
            ErrorMessage::AddressNotFound => "Ensure the addressId is correct and exists in the database. Use the 'GET /api/v1/addresses' endpoint to retrieve available address IDs.",
            ErrorMessage::CollaboratorExist => "Verify the collaborator details (email, cpf) are unique and do not already exist.",
            ErrorMessage::CollaboratorNotFound => "Ensure the collaboratorId, email or cpf is correct and exists in the database. Use the 'GET /api/v1/collaborators' endpoint to retrieve available collaborator IDs.",
            ErrorMessage::VehicleExist => "Ensure the vehicle information are uique and do not already exist.",
            ErrorMessage::VehicleNotFound => "Ensure the vehicleId is correct and exists in the database. Use the 'GET /api/v1/vehicles' endpoint to retrive available vehicle IDs",
            ErrorMessage::VehicleDocumentExist => "Verify the vehicle document details (chassisNumber, registrationNumber or plate) are unique and do not already exist.",
            ErrorMessage::VehicleDocumentNotFound => "Ensure the vehicleId, chassisNumber, registrationNumber or plate is correct and exists in the database. Use the 'GET /api/v1/vehicles' endpoint to retrieve available vehicle IDs and the 'GET /api/v1/vehicle{vehicleId}/documents' to retrieve the vehicle document.",
            ErrorMessage::RouteNotFound => "Ensure the routeId is correct and exists in the database. Use the 'GET /api/v1/routes' endpoint to retrieve available route IDs.",
        }
    }
}

#[derive(Debug, Clone)]
pub struct HttpError {
    pub status: u16,
    pub message: String,
    pub hint: String,
}

impl HttpError {
    pub fn server_error(message: impl Into<String>) -> Self {
        HttpError {
            message: message.into(),
            hint: ErrorMessage::ServerError.hint().to_string(),
            status: 500,
        }
    }

    pub fn bad_request(message: impl Into<String>) -> Self {
        HttpError {
            message: message.into(),
            hint: "Check the request parameters and try again.".to_string(),
            status: 400,
        }
    }

    pub fn unique_constraint_violation(message: impl Into<String>) -> Self {
        HttpError {
            message: message.into(),
            hint: "Ensure the data you are trying to add is unique.".to_string(),
            status: 409,
        }
    }

    pub fn from_error_message(error_message: ErrorMessage) -> Self {
        HttpError {
            message: error_message.to_string(),
            hint: error_message.hint().to_string(),
            status: match error_message {
                ErrorMessage::ServerError => 500,
                ErrorMessage::CountryExist
                | ErrorMessage::StateExist
                | ErrorMessage::CityExist
                | ErrorMessage::AddressExist => 409,
                _ => 404,
            },
        }
    }

    pub fn into_http_response(self) -> HttpResponse {
        let response = Response {
            error: ResponseDetails {
                status: match self.status {
                    400 | 409 => "fail".to_string(),
                    _ => "error".to_string(),
                },
                code: self.status.to_string(),
                message: self.message,
                hint: self.hint,
            },
        };

        match self.status {
            400 => HttpResponse::BadRequest().json(response),
            401 => HttpResponse::Unauthorized().json(response),
            404 => HttpResponse::NotFound().json(response),
            409 => HttpResponse::Conflict().json(response),
            500 => HttpResponse::InternalServerError().json(response),
            _ => {
                eprintln!(
                    "Warning: Missing pattern match. Converted status code {} to 500.",
                    self.status
                );

                HttpResponse::InternalServerError().json(Response {
                    error: ResponseDetails {
                        status: "error".to_string(),
                        code: "500".to_string(),
                        message: ErrorMessage::ServerError.to_string(),
                        hint: ErrorMessage::ServerError.hint().to_string(),
                    },
                })
            }
        }
    }
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "HttpError: message: {}, status: {}",
            self.message, self.status
        )
    }
}

impl std::error::Error for HttpError {}

impl ResponseError for HttpError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        let cloned = self.clone();

        cloned.into_http_response()
    }
}
