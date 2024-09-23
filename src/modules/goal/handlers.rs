use std::sync::Arc;
use axum::{
    extract::{Json, Path, State},
    middleware,
    response::IntoResponse,
    routing::{post, put},
    Extension, Router,
};
use mongodb::bson::oid::ObjectId;
use validator::Validate;

use crate::{
    helpers::api_response::ApiResponse,
    modules::{auth::{self, dto::AuthState}, category::{dto::CategoryResponse, repository::CategoryRepository}, goal::{dto::{CreateGoalRequest, UpdateGoalRequest}, repository::GoalRepository, service::{GoalService, GoalServiceError}}},
    AppState,
};

use super::dto::GoalResponse;

async fn create_goal(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<AuthState>,
    Json(payload): Json<CreateGoalRequest>,
) -> impl IntoResponse {
    if let Err(errors) = payload.validate() {
        return ApiResponse::bad_request("Validation failed", Some(errors)).into_response();
    }

    let repository = GoalRepository::new(&state.mongodb);
    let service = GoalService::new(repository);

    match service.create_goal_for_user(user.id, payload).await {
        Ok(id) => ApiResponse::created("Goal created successfully", Some(id.to_string())).into_response(),
        Err(GoalServiceError::GoalAlreadyExists) => ApiResponse::unprocessable_entity(
            GoalServiceError::GoalAlreadyExists.to_string().as_str(),
            None::<()>,
        ).into_response(),
        Err(err) => ApiResponse::server_error(Some(err.to_string().as_str()), None::<()>).into_response(),
    }
}

async fn update_goal(
    Path(goal_id): Path<ObjectId>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<AuthState>,
    Json(payload): Json<UpdateGoalRequest>,
) -> impl IntoResponse {
    if let Err(errors) = payload.validate() {
        return ApiResponse::bad_request("Validation failed", Some(errors)).into_response();
    }

    let repository = GoalRepository::new(&state.mongodb);
    let service = GoalService::new(repository);

    match service.update_user_goal(user.id, goal_id, payload).await {
        Ok(goal) => ApiResponse::ok("Goal updated successfully", Some(goal)).into_response(),
        Err(GoalServiceError::GoalNotFound) => ApiResponse::not_found("Goal not found").into_response(),
        Err(err) => ApiResponse::server_error(Some(err.to_string().as_str()), None::<()>).into_response(),
    }
}

async fn delete_goal(
    Path(goal_id): Path<ObjectId>,
    Extension(user): Extension<AuthState>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let repository = GoalRepository::new(&state.mongodb);
    let service = GoalService::new(repository);

    match service.delete_user_goal(user.id, goal_id).await {
        Ok(_) => ApiResponse::ok("Goal deleted successfully", None::<()>).into_response(),
        Err(GoalServiceError::GoalNotFound) => ApiResponse::not_found("Goal not found").into_response(),
        Err(err) => ApiResponse::server_error(Some(err.to_string().as_str()), None::<()>).into_response(),
    }
}

async fn list_goals(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<AuthState>,
) -> impl IntoResponse {
    let repository = GoalRepository::new(&state.mongodb);
    let category_repository = CategoryRepository::new(&state.mongodb);
    let service = GoalService::new(repository);

    match service.get_all_user_goals(&user.id).await {
        Ok(goals) => {
            let mut response_goals = Vec::new();
            let category_result = category_repository.get_all_user_categories(&user.id).await;
            let categories = match category_result {
                Ok(cats) => cats,
                Err(_) => Vec::new(),
            };

            for goal in goals {
                let category_response = categories
                    .iter()
                    .find(|cat| cat.id == goal.category_id)
                    .map(|category| CategoryResponse {
                        _id: category.id.unwrap().to_string(),
                        title: category.title.clone(),
                        color: category.color.clone(),
                    });

                response_goals.push(GoalResponse {
                    _id: goal.id.unwrap().to_string(),
                    title: goal.title,
                    description: goal.description,
                    end_date: goal.end_date,
                    status: goal.status,
                    category: category_response,
                    priority: goal.priority,
                });
            }

            ApiResponse::ok("Goals retrieved successfully", Some(response_goals)).into_response()
        }
        Err(err) => ApiResponse::server_error(Some(err.to_string().as_str()), None::<()>).into_response(),
    }
}

pub fn handles() -> Router<Arc<AppState>> {
    Router::new()
        .route("/v1/goals", post(create_goal).get(list_goals))
        .route("/v1/goals/:goal_id", put(update_goal).delete(delete_goal))
        .layer(middleware::from_fn(auth::middlewares::authorize))
}