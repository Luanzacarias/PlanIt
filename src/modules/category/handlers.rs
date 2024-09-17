use axum::{
    extract::Path,
    extract::{Json, State},
    middleware,
    response::IntoResponse,
    routing::delete,
    routing::post,
    routing::put,
    Extension, Router,
};
use mongodb::bson::oid::ObjectId;
use std::sync::Arc;
use validator::Validate;

use crate::{helpers::api_response, AppState};
use crate::{
    helpers::api_response::ApiResponse,
    modules::auth::{self, dto::AuthState},
};

use super::service::{CategoryService, CategoryServiceError};
use super::{dto::UpdateCategoryRequest, repository::CategoryRepository};
use super::{
    dto::{CategoryResponse, CreateCategoryRequest},
    repository,
};

async fn delete_category(
    State(state): State<Arc<AppState>>,
    Path(category_id): Path<String>,
    Extension(user): Extension<AuthState>,
) -> impl IntoResponse {
    let repository = CategoryRepository::new(&state.mongodb);
    let service = CategoryService::new(repository);

    let category_id = ObjectId::parse_str(&category_id).expect("Invalid ObjectId");
    match service.delete_user_category(category_id, &user.id).await {
        Ok(_) => ApiResponse::ok("Category deleted successfully", None::<()>).into_response(),
        Err(err) => ApiResponse::server_error(
            Some(format!("Failed to delete category: {}", err).as_str()),
            None::<()>,
        )
        .into_response(),
    }
}

async fn create_category(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<AuthState>,
    Json(mut payload): Json<CreateCategoryRequest>,
) -> impl IntoResponse {
    payload.title = payload.title.trim().to_string();

    if let Err(errors) = payload.validate() {
        return ApiResponse::bad_request("Validation failed", Some(errors)).into_response();
    }

    let repository = CategoryRepository::new(&state.mongodb);
    let service = CategoryService::new(repository);

    match service
        .create_category_for_user(&user.id, payload.title.clone(), payload.color)
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

async fn update_category(
    State(state): State<Arc<AppState>>,
    Path(category_id): Path<String>,
    Extension(user): Extension<AuthState>,
    Json(payload): Json<UpdateCategoryRequest>,
) -> impl IntoResponse {
    let repository = CategoryRepository::new(&state.mongodb);
    let service = CategoryService::new(repository);

    let category_id = ObjectId::parse_str(&category_id).expect("Invalid ObjectId");

    if let Err(errors) = payload.validate() {
        return ApiResponse::bad_request("Validation failed", Some(errors)).into_response();
    }

    match service
        .update_category(&user.id, category_id, payload.title, payload.color)
        .await
    {
        Ok(_) => ApiResponse::ok("Category updated successfully", None::<()>).into_response(),
        Err(CategoryServiceError::CategoryNotFound) => ApiResponse::unprocessable_entity(
            CategoryServiceError::CategoryNotFound.to_string().as_str(),
            None::<()>,
        )
        .into_response(),
        Err(err) => {
            ApiResponse::server_error(Some(err.to_string().as_str()), None::<()>).into_response()
        }
    }
}

async fn get_categories(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<AuthState>,
) -> impl IntoResponse {
    let repository = CategoryRepository::new(&state.mongodb);
    let service = CategoryService::new(repository);

    match service.get_all_user_categories(&user.id).await {
        Ok(categories) => {
            let response_categories: Vec<_> = categories
                .into_iter()
                .map(|category| CategoryResponse {
                    _id: category.id.unwrap().to_string(),
                    title: category.title,
                    color: category.color,
                })
                .collect();

            ApiResponse::ok(
                "Categories retrieved successfully",
                Some(response_categories),
            )
            .into_response()
        }
        Err(err) => {
            ApiResponse::server_error(Some(err.to_string().as_str()), None::<()>).into_response()
        }
    }
}

pub fn handles() -> Router<Arc<AppState>> {
    Router::new()
        .route("/v1/categories", post(create_category).get(get_categories))
        .route(
            "/v1/categories/:category_id",
            delete(delete_category).put(update_category),
        )
        .layer(middleware::from_fn(auth::middlewares::authorize))
}
