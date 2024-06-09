use actix_web::HttpRequest;
use regex::Regex;
use uuid::Uuid;
use validator::ValidationError;

use crate::error::HttpError;

pub fn is_valid_uuid(city_id: &str) -> Result<(), ValidationError> {
    match Uuid::parse_str(city_id) {
        Ok(_) => Ok(()),
        Err(_) => Err(ValidationError::new("Is not a valid UUID")),
    }
}

pub fn extract_endpoint_from_path(regex: &str, request: &HttpRequest) -> Result<String, HttpError> {
    let path = request.path();
    let re = Regex::new(regex)
        .map_err(|e| HttpError::server_error(format!("Failed to compile regex: {}", e)))?;

    let endpoint = re
        .captures(path)
        .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()));

    match endpoint {
        Some(endpoint) => Ok(endpoint),
        None => Err(HttpError::bad_request(
            "No endpoint found in request path".to_string(),
        )),
    }
}
