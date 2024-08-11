use actix_web::HttpResponse;
use serde::Serialize;

#[derive(Serialize)]
pub struct SuccessApiResponse<T> {
    pub message: String,
    pub data: Option<T>,
}

impl<T> SuccessApiResponse<T> {
    pub fn new(message: String, data: Option<T>) -> Self {
        SuccessApiResponse { message, data }
    }
}

#[derive(Serialize)]
pub struct ErrorApiResponse<T> {
    pub message: String,
    pub errors: Option<T>,
}

impl<T> ErrorApiResponse<T> {
    pub fn new(message: String, errors: Option<T>) -> Self {
        ErrorApiResponse { message, errors }
    }

    pub fn from_validation_errors(errors: T) -> Self {
        ErrorApiResponse {
            message: "Validation error".to_string(),
            errors: Some(errors),
        }
    }
}


pub fn api_success<T: serde::Serialize>(message: &str, data: Option<T>) -> HttpResponse {
    HttpResponse::Ok().json(SuccessApiResponse::new(message.into(), data))
}

pub fn api_unknown_error(err: &str) -> HttpResponse {
    HttpResponse::InternalServerError().json(ErrorApiResponse::new(err.into(), None::<()>))
}

pub fn api_validation_error(errors: validator::ValidationErrors) -> HttpResponse {
    HttpResponse::BadRequest().json(ErrorApiResponse::from_validation_errors(errors))
}
