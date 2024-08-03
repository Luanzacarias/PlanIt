use actix_web::{get, post, web, HttpResponse, Responder};
use serde::Deserialize;

use crate::category::repository::CategoryRepository;
use crate::category::service::CategoryService;
use crate::AppState;

#[derive(Deserialize)]
struct CreateCategoryRequest {
    title: String,
    description: String,
}

#[post("/v1/categories")]
async fn create_category(
    state: web::Data<AppState>,
    item: web::Json<CreateCategoryRequest>,
) -> impl Responder {
    let mut item: CreateCategoryRequest = item.into_inner();
    item.title = item.title.trim().to_string();
    if item.title.is_empty() {
        return HttpResponse::BadRequest().json("Title cannot be empty");
    }

    let repository = CategoryRepository::new(&state.mongodb);
    let service = CategoryService::new(repository);
    match service
        .create_category(item.title.clone(), item.description.clone())
        .await
    {
        Ok(id) => HttpResponse::Ok().json(id.to_string()),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

#[get("/v1/categories")]
async fn get_categories(state: web::Data<AppState>) -> impl Responder {
    let repository = CategoryRepository::new(&state.mongodb);
    let service = CategoryService::new(repository);

    match service.get_all_categories().await {
        Ok(categories) => HttpResponse::Ok().json(categories),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

pub fn handles(cfg: &mut web::ServiceConfig) {
    cfg.service(create_category).service(get_categories);
}
