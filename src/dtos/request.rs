use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct RequestQueryDTO {
    #[validate(range(min = 1))]
    pub page: Option<usize>,

    #[validate(range(min = 1, max = 50))]
    pub limit: Option<usize>,
}
