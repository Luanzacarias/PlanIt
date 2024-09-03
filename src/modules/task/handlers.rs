use axum::{
    extract::{Json, State},
    middleware,
    response::IntoResponse,
    routing::post,
    Extension, Router,
};
use std::sync::Arc;
use validator::Validate;

use crate::{modules::category::{dto::CategoryResponse, repository::CategoryRepository}, AppState};
use crate::{
    helpers::api_response::ApiResponse,
    modules::auth::{self, dto::AuthState},
};

use super::dto::{TaskResponse, CreateTaskRequest};
use super::repository::TaskRepository;
use super::service::{TaskService, TaskServiceError};

async fn create_task(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<AuthState>,
    Json(mut payload): Json<CreateTaskRequest>,
) -> impl IntoResponse {
    payload.title = payload.title.trim().to_string();

    if let Err(errors) = payload.validate() {
        return ApiResponse::bad_request("Validation failed", Some(errors)).into_response();
    }

    let repository = TaskRepository::new(&state.mongodb);
    let service = TaskService::new(repository);

    match service
        .create_task_for_user(payload.title.clone(), payload.description.clone(), payload.start_date.clone(), payload.end_date.clone(), payload.status, &user.id, &payload.category_id)
        .await
    {
        Ok(id) => ApiResponse::created("Task created successfully", Some(id.to_string()))
            .into_response(),
        Err(TaskServiceError::TaskAlreadyExists) => ApiResponse::unprocessable_entity(
            TaskServiceError::TaskAlreadyExists
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

async fn get_tasks(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<AuthState>,
) -> impl IntoResponse {
    let repository = TaskRepository::new(&state.mongodb);
    let service = TaskService::new(repository);

    let category_repository = CategoryRepository::new(&state.mongodb);

    match service.get_all_user_tasks(&user.id).await {
        Ok(tasks) => {
            let mut response_tasks = Vec::new();

            for task in tasks {
                let category_result = category_repository.get_category_by_id(&user.id, &task.category_id).await;

                let category_response = match category_result {
                    Ok(Some(category)) => Some(CategoryResponse {
                        _id: category.id.unwrap().to_string(),
                        title: category.title,
                        color: category.color,
                    }),
                    _ => None,
                };

                response_tasks.push(TaskResponse {
                    _id: task.id.unwrap().to_string(),
                    title: task.title,
                    description: task.description,
                    start_date: task.start_date,
                    end_date: task.end_date,
                    status: task.status,
                    category: category_response,
                });
            }

            Json(ApiResponse::ok("Tasks retrieved successfully", Some(response_tasks))).into_response()
        }
        Err(err) => Json(ApiResponse::server_error(Some(&err.to_string()), None::<()>)).into_response(),
    }
}

pub fn handles() -> Router<Arc<AppState>> {
    Router::new()
        .route("/v1/tasks", post(create_task).get(get_tasks))
        .layer(middleware::from_fn(auth::middlewares::authorize))
}
