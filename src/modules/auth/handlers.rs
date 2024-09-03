use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, routing::post, Json, Router};
use tower_http::cors::CorsLayer;

use crate::{
    helpers::api_response::ApiResponse,
    modules::user::{repository::UserRepository, service::UserService},
    AppState,
};

use super::{dto::UserLoginRequest, service::AuthService};

async fn login(
    State(state): State<Arc<AppState>>,
    Json(mut payload): Json<UserLoginRequest>,
) -> impl IntoResponse {
    let user_service = UserService::new(UserRepository::new(&state.mongodb));
    let auth_service = AuthService::new(user_service);

    payload.email = payload.email.trim().to_string();
    match auth_service.login(&payload.email, &payload.password).await {
        Ok(res) => ApiResponse::ok("Login successful", Some(res)).into_response(),
        Err(err) => {
            ApiResponse::server_error(Some(err.to_string().as_str()), None::<()>).into_response()
        }
    }
}

pub fn handles() -> Router<Arc<AppState>> {
    let v1: Router<Arc<AppState>> = Router::new().route("/login", post(login));
    Router::new().nest("/v1", v1)
}
