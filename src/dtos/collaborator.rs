use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::models::collaborator::{self, Collaborator};

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct RegisterCollaboratorDTO {
    #[validate(length(
        min = 1,
        max = 100,
        message = "Collaborator name must have a maximum of 100 characters"
    ))]
    pub name: String,

    #[validate(length(
        min = 1,
        max = 11,
        message = "Collaborator CPF must have a maximum of 11 characters"
    ))]
    pub cpf: String,

    #[validate(length(
        min = 1,
        max = 9,
        message = "Collaborator CPF must have a maximum of 9 characters"
    ))]
    pub rg: String,

    #[validate(email)]
    pub email: String,

    #[serde(rename = "updatedAt")]
    pub updated_at: String,

    #[serde(rename = "createdAt")]
    pub crated_at: String,
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct FilterCollaboratorDTO {
    pub id: String,
    pub name: String,
    pub cpf: String,
    pub rg: String,
    pub email: String,

    #[serde(rename = "updatedAt")]
    pub updated_at: String,

    #[serde(rename = "createdAt")]
    pub crated_at: String,
}

impl FilterCollaboratorDTO {
    pub fn filter_collaborator(collaborator: &Collaborator) -> Self {
        FilterCollaboratorDTO {
            id: collaborator.id.to_string(),
            name: collaborator.name.to_owned(),
            cpf: collaborator.cpf.to_owned(),
            rg: collaborator.rg.to_owned(),
            email: collaborator.email.to_owned(),
            updated_at: collaborator.updated_at.to_owned(),
            crated_at: collaborator.created_at.to_owned(),
        }
    }

    pub fn filter_collaborators(collaborators: &[Collaborator]) -> Vec<FilterCollaboratorDTO> {
        collaborators
            .iter()
            .map(FilterCollaboratorDTO::filter_collaborator)
            .collect()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CollaboratorListResponseDTO {
    pub collaborators: Vec<FilterCollaboratorDTO>,
    pub results: usize,
}
