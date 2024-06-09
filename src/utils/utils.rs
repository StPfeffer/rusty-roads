use uuid::Uuid;
use validator::ValidationError;

pub fn is_valid_uuid(city_id: &str) -> Result<(), ValidationError> {
    match Uuid::parse_str(city_id) {
        Ok(_) => Ok(()),
        Err(_) => Err(ValidationError::new("Is not a valid UUID")),
    }
}
