use std::sync::Arc;
use axum::{
    extract::{Json, Path, State},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use mongodb::bson::oid::ObjectId;
use validator::Validate;

use crate::{
    helpers::api_response::ApiResponse,
    modules::goal::{dto::{CreateGoalRequest, UpdateGoalRequest}, repository::GoalRepository, service::{GoalService, GoalServiceError}},
    AppState,
};

async fn create_goal(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateGoalRequest>,
) -> impl IntoResponse {
    if let Err(errors) = payload.validate() {
        return ApiResponse::bad_request("Validation failed", Some(errors)).into_response();
    }

    let repository = GoalRepository::new(&state.mongodb);
    let service = GoalService::new(repository);

    match service.create_goal(payload).await {
        Ok(goal) => ApiResponse::created("Goal created successfully", Some(goal)).into_response(),
        Err(GoalServiceError::GoalAlreadyExists) => ApiResponse::unprocessable_entity(
            GoalServiceError::GoalAlreadyExists.to_string().as_str(),
            None::<()>,
        ).into_response(),
        Err(err) => ApiResponse::server_error(Some(err.to_string().as_str()), None::<()>).into_response(),
    }
}

async fn get_goal(
    Path(goal_id): Path<ObjectId>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let repository = GoalRepository::new(&state.mongodb);
    let service = GoalService::new(repository);

    match service.get_goal_by_id(&state.user_id, &goal_id).await {
        Ok(Some(goal)) => ApiResponse::ok("Goal retrieved successfully", Some(goal)).into_response(),
        Ok(None) => ApiResponse::not_found("Goal not found").into_response(),
        Err(err) => ApiResponse::server_error(Some(err.to_string().as_str()), None::<()>).into_response(),
    }
}

async fn update_goal(
    Path(goal_id): Path<ObjectId>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UpdateGoalRequest>,
) -> impl IntoResponse {
    if let Err(errors) = payload.validate() {
        return ApiResponse::bad_request("Validation failed", Some(errors)).into_response();
    }

    let repository = GoalRepository::new(&state.mongodb);
    let service = GoalService::new(repository);

    match service.update_goal(goal_id, payload).await {
        Ok(goal) => ApiResponse::ok("Goal updated successfully", Some(goal)).into_response(),
        Err(GoalServiceError::GoalNotFound) => ApiResponse::not_found("Goal not found").into_response(),
        Err(err) => ApiResponse::server_error(Some(err.to_string().as_str()), None::<()>).into_response(),
    }
}

async fn delete_goal(
    Path(goal_id): Path<ObjectId>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let repository = GoalRepository::new(&state.mongodb);
    let service = GoalService::new(repository);

    match service.delete_user_goal(goal_id, &state.user_id).await {
        Ok(_) => ApiResponse::ok("Goal deleted successfully", None::<()>).into_response(),
        Err(GoalServiceError::GoalNotFound) => ApiResponse::not_found("Goal not found").into_response(),
        Err(err) => ApiResponse::server_error(Some(err.to_string().as_str()), None::<()>).into_response(),
    }
}

async fn list_goals(
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let repository = GoalRepository::new(&state.mongodb);
    let service = GoalService::new(repository);

    match service.get_all_goals().await {
        Ok(goals) => ApiResponse::ok("Goals retrieved successfully", Some(goals)).into_response(),
        Err(err) => ApiResponse::server_error(Some(err.to_string().as_str()), None::<()>).into_response(),
    }
}

pub fn handles() -> Router<Arc<AppState>> {
    Router::new()
        .route("/v1/goals", post(create_goal).get(list_goals))
        .route("/v1/goals/:goal_id", get(get_goal).put(update_goal).delete(delete_goal))
}
