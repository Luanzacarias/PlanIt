use axum::extract::{Query, State};
use axum::http::{Response, StatusCode};
use axum::response::IntoResponse;
use axum::{extract::Json, routing::{post, get}, Router};
use std::sync::Arc;
use validator::Validate;

use crate::helpers::api_response::ApiResponse;
use crate::AppState;

use super::dto::{UserSignUpRequest, UserExistsQuery};
use super::repository::UserRepository;
use super::service::{UserService, UserServiceError};

async fn sign_up(
    State(state): State<Arc<AppState>>,
    Json(mut payload): Json<UserSignUpRequest>,
) -> impl IntoResponse {
    payload.name = payload.name.trim().to_string();
    payload.email = payload.email.trim().to_string();
    payload.phone = payload.phone.trim().to_string();

    if let Err(errors) = payload.validate() {
        return ApiResponse::bad_request("Validation failed", Some(errors)).into_response();
    }

    match UserService::new(UserRepository::new(&state.mongodb))
        .create_user(payload)
        .await
    {
        Ok(id) => {
            ApiResponse::created("User created successfully", Some(id.to_string())).into_response()
        }
        Err(UserServiceError::UserAlreadyExists) => ApiResponse::unprocessable_entity(
            UserServiceError::UserAlreadyExists.to_string().as_str(),
            None::<()>,
        )
        .into_response(),
        Err(err) => {
            ApiResponse::server_error(Some(err.to_string().as_str()), None::<()>).into_response()
        }
    }
}

async fn user_exists(
    State(state): State<Arc<AppState>>,
    Query(query): Query<UserExistsQuery>,
) -> impl IntoResponse {
    let email = query.email.trim();

    match UserService::new(UserRepository::new(&state.mongodb))
        .find_user_by_email(email)
        .await
    {
        Ok(Some(_user)) => {
            ApiResponse::ok("User exists", Some(true)).into_response()
        }
        Ok(None) => {
            Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body("User not found".into())
                .unwrap()
        }
        Err(err) => {
            ApiResponse::server_error(Some(err.to_string().as_str()), None::<()>).into_response()
        }
    }
}

pub fn handles() -> Router<Arc<AppState>> {
    let v1: Router<Arc<AppState>> = Router::new()
        .route("/signup", post(sign_up))
        .route("/user-exists", get(user_exists));
    Router::new().nest("/v1", v1)
}
