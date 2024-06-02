use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{models::state::State, utils::uuid::is_valid_uuid};

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterStateDTO {
    #[validate(length(
        min = 1,
        max = 100,
        message = "Name must have a maximum of 100 characters"
    ))]
    pub name: String,

    #[validate(length(min = 2, max = 2, message = "Code must be 2 characters long"))]
    pub code: String,

    #[validate(custom(
        function = "is_valid_uuid",
        message = "Country ID must be a valid UUID"
    ))]
    pub country_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilterStateDTO {
    pub id: String,
    pub name: String,
    pub code: String,
    pub country_id: String,
}

impl FilterStateDTO {
    pub fn filter_state(state: &State) -> Self {
        FilterStateDTO {
            id: state.id.to_string(),
            name: state.name.to_owned(),
            code: state.code.to_owned(),
            country_id: state.country_id.to_string(),
        }
    }

    pub fn filter_states(states: &[State]) -> Vec<FilterStateDTO> {
        states.iter().map(FilterStateDTO::filter_state).collect()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StateResponseDTO {
    pub status: String,
    pub data: FilterStateDTO,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StateListResponseDTO {
    pub states: Vec<FilterStateDTO>,
    pub results: usize,
}
