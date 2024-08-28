mod config;
mod helpers;
mod modules;

use axum::{extract::Json, routing::get, Router};
use dotenv::dotenv;
use env_logger::Env;
use log::info;
use modules::{category, user};
use mongodb::Database;
use std::env;
use std::sync::Arc;
use tokio::net::TcpListener;

const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

struct AppState {
    mongodb: Database,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let app_host: String = env::var("APP_HOST").unwrap_or_else(|_| String::from("127.0.0.1:8080"));
    let mongodb = config::mongodb::get_database().await;

    let listener = TcpListener::bind(app_host)
        .await
        .expect("Unable to connect to the server");

    info!("Web Server running at {}", listener.local_addr().unwrap());

    let state = Arc::new(AppState { mongodb });
    let app = Router::new()
        .route("/", get(|| async { Json(format!("PlanIt v{}", VERSION.unwrap_or("unknown"))) }))
        .nest("/", user::handles())
        .nest("/", category::handles())
        .with_state(state);

    axum::serve(listener, app)
        .await
        .expect("Error serving application");
}
