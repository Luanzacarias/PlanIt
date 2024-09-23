use crate::{
    helpers::api_response::ApiResponse,
    modules::{
        auth::{self, dto::AuthState},
        task::repository::TaskRepository,
    },
    AppState,
};
use axum::{
    extract::State,
    middleware,
    response::IntoResponse,
    routing::get,
    Extension, Router,
};
use std::sync::Arc;

async fn get_notifications(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<AuthState>,
) -> impl IntoResponse {
    let task_repository = TaskRepository::new(&state.mongodb);

    match task_repository.get_all_with_notifications(&user.id).await {
        Ok(notifications) => {
            ApiResponse::ok("Notifications retrieved successfully", Some(notifications))
                .into_response()
        }
        Err(err) => ApiResponse::server_error(
            Some("Failed to retrieve notifications"),
            Some(err.to_string()),
        )
        .into_response(),
    }
}

pub fn handles() -> Router<Arc<AppState>> {
    Router::new()
        .route("/v1/notifications", get(get_notifications))
        .layer(middleware::from_fn(auth::middlewares::authorize))
}
