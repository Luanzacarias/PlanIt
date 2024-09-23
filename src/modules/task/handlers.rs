use crate::{
    helpers::api_response::ApiResponse,
    modules::auth::{self, dto::AuthState},
    modules::category::{dto::CategoryResponse, repository::CategoryRepository},
    AppState,
};

use axum::{
    extract::{Json, Path, State},
    middleware,
    response::IntoResponse,
    routing::{post, put},
    Extension, Router,
};
use mongodb::bson::oid::ObjectId;
use std::sync::Arc;
use validator::Validate;

use super::dto::{CreateTaskRequest, TaskResponse, UpdateTaskRequest};
use super::repository::TaskRepository;
use super::service::{TaskService, TaskServiceError};

async fn create_task(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<AuthState>,
    Json(payload): Json<CreateTaskRequest>,
) -> impl IntoResponse {
    if let Err(errors) = payload.validate() {
        return ApiResponse::bad_request("Validation failed", Some(errors)).into_response();
    }

    let repository = TaskRepository::new(&state.mongodb);
    let service = TaskService::new(repository);

    match service.create_task_for_user(&user.id, payload).await {
        Ok(id) => {
            ApiResponse::created("Task created successfully", Some(id.to_string())).into_response()
        }
        Err(TaskServiceError::TaskAlreadyExists) => ApiResponse::unprocessable_entity(
            TaskServiceError::TaskAlreadyExists.to_string().as_str(),
            None::<()>,
        )
        .into_response(),
        Err(err) => {
            ApiResponse::server_error(Some(err.to_string().as_str()), None::<()>).into_response()
        }
    }
}

async fn update_task(
    Path(task_id): Path<ObjectId>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<AuthState>,
    Json(payload): Json<UpdateTaskRequest>,
) -> impl IntoResponse {
    if let Err(errors) = payload.validate() {
        return ApiResponse::bad_request("Validation failed", Some(errors)).into_response();
    }

    let repository = TaskRepository::new(&state.mongodb);
    let service = TaskService::new(repository);

    match service
        .update_user_task(
            &user.id,
            &task_id,
            payload,
        )
        .await
    {
        Ok(result) => {
            Json(ApiResponse::ok("Task updated successfully", Some(result))).into_response()
        }
        Err(TaskServiceError::TaskNotFound) => ApiResponse::bad_request(
            TaskServiceError::TaskNotFound.to_string().as_str(),
            None::<()>,
        )
        .into_response(),
        Err(TaskServiceError::TaskAlreadyExists) => ApiResponse::unprocessable_entity(
            TaskServiceError::TaskAlreadyExists.to_string().as_str(),
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

            let category_result = category_repository.get_all_user_categories(&user.id).await;

            let categories = match category_result {
                Ok(cats) => cats,
                Err(_) => Vec::new(),
            };

            for task in tasks {
                let category_response = categories
                    .iter()
                    .find(|cat| cat.id == Some(task.category_id))
                    .map(|category| CategoryResponse {
                        _id: category.id.unwrap().to_string(),
                        title: category.title.clone(),
                        color: category.color.clone(),
                    });

                response_tasks.push(TaskResponse {
                    _id: task.id.unwrap().to_string(),
                    title: task.title,
                    description: task.description,
                    start_date: task.start_date,
                    end_date: task.end_date,
                    status: task.status,
                    category: category_response,
                    notification_time_unit: task.notification.as_ref().map(|n| n.time_unit.clone()),
                    notification_time_value: task.notification.as_ref().map(|n| n.time_value),
                });
            }

            Json(ApiResponse::ok(
                "Tasks retrieved successfully",
                Some(response_tasks),
            ))
            .into_response()
        }
        Err(err) => Json(ApiResponse::server_error(
            Some(&err.to_string()),
            None::<()>,
        ))
        .into_response(),
    }
}

async fn delete_task(
    Path(task_id): Path<ObjectId>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let repository = TaskRepository::new(&state.mongodb);
    let service = TaskService::new(repository);

    match service.delete_user_task(&task_id).await {
        Ok(result) => {
            Json(ApiResponse::ok("Task deleted successfully", Some(result))).into_response()
        }
        Err(TaskServiceError::TaskNotFound) => ApiResponse::bad_request(
            TaskServiceError::TaskNotFound.to_string().as_str(),
            None::<()>,
        )
        .into_response(),
        Err(err) => {
            ApiResponse::server_error(Some(err.to_string().as_str()), None::<()>).into_response()
        }
    }
}

pub async fn get_task_stats(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<AuthState>,
) -> impl IntoResponse {
    // Inicializa o repositório e serviço
    let repository = TaskRepository::new(&state.mongodb);
    let service = TaskService::new(repository);

    // Chama o serviço para obter as tarefas por categoria e status
    match service.count_tasks_by_category_and_status(&user.id).await {
        Ok(task_stats) => Json(ApiResponse::ok(
            "Tasks by category and status retrieved successfully",
            Some(task_stats),
        ))
        .into_response(),
        Err(err) => Json(ApiResponse::server_error(
            Some(&err.to_string()),
            None::<()>,
        ))
        .into_response(),
    }
}

pub fn handles() -> Router<Arc<AppState>> {
    Router::new()
        .route("/v1/tasks", post(create_task).get(get_tasks))
        .route("/v1/tasks/:task_id", put(update_task).delete(delete_task))
        .route("/v1/tasks/categories", get(get_task_stats))
        .layer(middleware::from_fn(auth::middlewares::authorize))
}
