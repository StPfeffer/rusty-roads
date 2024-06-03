use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::models::vehicle::Vehicle;
use crate::{models::vehicle::VehicleDocument, utils::uuid::is_valid_uuid};

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterVehicleDTO {
    #[validate(length(
        min = 1,
        max = 50,
        message = "Name must have a maximum of 50 characters"
    ))]
    pub name: String,

    pub initial_mileage: i32,
    pub actual_mileage: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilterVehicleDTO {
    pub id: String,
    pub name: String,
    pub initial_mileage: i32,
    pub actual_mileage: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl FilterVehicleDTO {
    pub fn filter_vehicle(vehicle: &Vehicle) -> Self {
        FilterVehicleDTO {
            id: vehicle.id.to_string(),
            name: vehicle.name.to_owned(),
            initial_mileage: vehicle.initial_mileage.to_owned(),
            actual_mileage: vehicle.actual_mileage.to_owned(),
            created_at: vehicle.created_at.to_owned(),
            updated_at: vehicle.updated_at.to_owned(),
        }
    }

    pub fn filter_vehicles(states: &[Vehicle]) -> Vec<FilterVehicleDTO> {
        states
            .iter()
            .map(FilterVehicleDTO::filter_vehicle)
            .collect()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VehicleResponseDTO {
    pub status: String,
    pub data: FilterVehicleDTO,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VehicleListResponseDTO {
    pub vehicles: Vec<FilterVehicleDTO>,
    pub results: usize,
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterVehicleDocumentDTO {
    #[validate(length(
        min = 10,
        max = 17,
        message = "Chassis number must have between 10 and 17 characters"
    ))]
    pub chassis_number: String,

    pub exercise_year: i16,
    pub model_year: i16,
    pub manufacture_year: i16,

    #[validate(length(
        min = 10,
        max = 20,
        message = "Registration number must have between 10 and 20 characters"
    ))]
    pub registration_number: String,

    #[validate(length(
        min = 1,
        max = 60,
        message = "Color must have between 1 and 60 characters"
    ))]
    pub color: String,

    #[validate(length(
        min = 1,
        max = 60,
        message = "Make must have between 1 and 60 characters"
    ))]
    pub make: String,

    #[validate(length(
        min = 1,
        max = 60,
        message = "Model must have between 1 and 60 characters"
    ))]
    pub model: String,

    #[validate(length(
        min = 5,
        max = 7,
        message = "Plate must have between 5 and 7 characters"
    ))]
    pub plate: String,

    pub updated_at: NaiveDateTime,

    #[validate(custom(
        function = "is_valid_uuid",
        message = "Vehicle ID must be a valid UUID"
    ))]
    pub vehicle_id: String,
}

impl RegisterVehicleDocumentDTO {
    pub fn into_save_vehicle_document_params_dto(self) -> SaveVehicleDocumentParamsDTO<String> {
        SaveVehicleDocumentParamsDTO {
            chassis_number: self.chassis_number,
            exercise_year: self.exercise_year,
            model_year: self.model_year,
            manufacture_year: self.manufacture_year,
            registration_number: self.registration_number,
            color: self.color,
            make: self.make,
            model: self.model,
            plate: self.plate,
            vehicle_id: self.vehicle_id,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SaveVehicleDocumentParamsDTO<T> {
    pub chassis_number: T,
    pub exercise_year: i16,
    pub model_year: i16,
    pub manufacture_year: i16,
    pub registration_number: T,
    pub color: T,
    pub make: T,
    pub model: T,
    pub plate: T,
    pub vehicle_id: T,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilterVehicleDocumentDTO {
    pub id: String,
    pub chassis_number: String,
    pub exercise_year: i16,
    pub model_year: i16,
    pub manufacture_year: i16,
    pub registration_number: String,
    pub color: String,
    pub make: String,
    pub model: String,
    pub plate: String,
    pub updated_at: NaiveDateTime,
    pub vehicle_id: String,
}

impl FilterVehicleDocumentDTO {
    pub fn filter_document(document: &VehicleDocument) -> Self {
        FilterVehicleDocumentDTO {
            id: document.id.to_string(),
            chassis_number: document.chassis_number.to_owned(),
            exercise_year: document.exercise_year.to_owned(),
            model_year: document.model_year.to_owned(),
            manufacture_year: document.manufacture_year.to_owned(),
            registration_number: document.registration_number.to_owned(),
            color: document.color.to_owned(),
            make: document.make.to_owned(),
            model: document.model.to_owned(),
            plate: document.plate.to_owned(),
            updated_at: document.updated_at.to_owned(),
            vehicle_id: document.vehicle_id.to_string(),
        }
    }

    pub fn filter_documents(documents: &[VehicleDocument]) -> Vec<FilterVehicleDocumentDTO> {
        documents
            .iter()
            .map(FilterVehicleDocumentDTO::filter_document)
            .collect()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VehicleDocumentResponseDTO {
    pub status: String,
    pub data: FilterVehicleDocumentDTO,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VehicleDocumentListResponseDTO {
    pub documents: Vec<FilterVehicleDocumentDTO>,
    pub results: usize,
}
