use serde::{Deserialize, Serialize};
use validator::Validate;

use super::models::Color;

#[derive(Deserialize, Validate)]
pub struct CreateCategoryRequest {
    #[validate(length(min = 1, max = 30))]
    pub title: String,
    pub color: Color,
}

#[derive(Serialize)]
pub struct CategoryResponse {
    pub _id: String,
    pub title: String,
    pub color: Color,
}
