use serde::Deserialize;
use validator::Validate;

use super::models::Color;

#[derive(Deserialize, Validate)]
pub struct CreateCategoryRequest {
    #[validate(length(min = 1, max = 30))]
    pub title: String,
    pub color: Color,
}
