use actix_web::{post, web, HttpResponse, Responder};
use validator::Validate;

use crate::helpers::api_response::{ErrorApiResponse, SuccessApiResponse};
use crate::AppState;

use super::dto::UserSignUpRequest;
use super::repository::UserRepository;
use super::service::{ServiceError, UserService};

#[post("/v1/signup")]
async fn sign_up(state: web::Data<AppState>, item: web::Json<UserSignUpRequest>) -> impl Responder {
    let mut item: UserSignUpRequest = item.into_inner();
    item.name = item.name.trim().to_string();
    item.email = item.email.trim().to_string();
    item.phone = item.phone.trim().to_string();

    if let Err(errors) = item.validate() {
        return HttpResponse::BadRequest().json(ErrorApiResponse::from_validation_errors(errors));
    }

    match UserService::new(UserRepository::new(&state.mongodb))
        .create_user(item)
        .await
    {
        Ok(id) => HttpResponse::Created().json(SuccessApiResponse::new(
            "User created successfully".to_string(),
            Some(id.to_string()),
        )),
        Err(ServiceError::UserAlreadyExists) => HttpResponse::Conflict().json(
            ErrorApiResponse::new(ServiceError::UserAlreadyExists.to_string(), None::<()>),
        ),
        Err(err) => HttpResponse::InternalServerError()
            .json(ErrorApiResponse::new(err.to_string(), None::<()>)),
    }
}

pub fn handles(cfg: &mut web::ServiceConfig) {
    cfg.service(sign_up);
}
