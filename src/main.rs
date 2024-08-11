use actix_web::{middleware, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use env_logger::Env;
use log::info;
use mongodb::Database;
use std::env;

mod user;
mod category;
mod config;
mod helpers;

struct AppState {
    mongodb: Database,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let app_host: String = env::var("APP_HOST").unwrap_or_else(|_| String::from("127.0.0.1:8080"));
    let mongodb = config::get_mongodb().await;
    info!("Server running at http://{}", app_host);
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                mongodb: mongodb.clone(),
            }))
            .wrap(middleware::Logger::default())
            .service(index_handler)
            .configure(user::handles)
            .configure(category::handles)
    })
    .bind(app_host.clone())?
    .run()
    .await
}

#[actix_web::get("/")]
async fn index_handler() -> impl Responder {
    HttpResponse::Ok().json("PlanIt v0.1.0")
}
