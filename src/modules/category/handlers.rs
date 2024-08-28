use axum::{
    extract::{Json, State},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use mongodb::bson::oid::ObjectId;
use std::sync::Arc;
use validator::Validate;

use crate::helpers::api_response::ApiResponse;
use crate::AppState;

use super::dto::CreateCategoryRequest;
use super::repository::CategoryRepository;
use super::service::{CategoryService, CategoryServiceError};

async fn create_category(
    State(state): State<Arc<AppState>>,
    Json(mut payload): Json<CreateCategoryRequest>,
) -> impl IntoResponse {
    payload.title = payload.title.trim().to_string();

    if let Err(errors) = payload.validate() {
        return ApiResponse::bad_request("Validation failed", Some(errors)).into_response();
    }

    let user_id = ObjectId::new();
    let repository = CategoryRepository::new(&state.mongodb);
    let service = CategoryService::new(repository);

    match service
        .create_category(user_id, payload.title.clone(), payload.color)
        .await
    {
        Ok(id) => ApiResponse::created("Category created successfully", Some(id.to_string()))
            .into_response(),
        Err(CategoryServiceError::CategoryAlreadyExists) => ApiResponse::unprocessable_entity(
            CategoryServiceError::CategoryAlreadyExists
                .to_string()
                .as_str(),
            None::<()>,
        )
        .into_response(),
        Err(err) => {
            ApiResponse::server_error(Some(err.to_string().as_str()), None::<()>).into_response()
        }
    }
}

async fn get_categories(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let repository = CategoryRepository::new(&state.mongodb);
    let service = CategoryService::new(repository);

    match service.get_all_categories().await {
        Ok(categories) => {
            ApiResponse::ok("Categories retrieved successfully", Some(categories)).into_response()
        }
        Err(err) => {
            ApiResponse::server_error(Some(err.to_string().as_str()), None::<()>).into_response()
        }
    }
}

pub fn handles() -> Router<Arc<AppState>> {
    Router::new().route("/v1/categories", post(create_category).get(get_categories))
}
