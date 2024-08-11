use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct UserSignUpRequest {
    #[validate(length(min = 3))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
    #[validate(length(min = 10, max = 15))]
    pub phone: String,
}
