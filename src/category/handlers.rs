use actix_web::{get, post, web, Responder};
use mongodb::bson::oid::ObjectId;
use validator::Validate;

use crate::helpers::api_response::{api_success, api_unknown_error, api_validation_error};
use crate::AppState;

use super::dto::CreateCategoryRequest;
use super::repository::CategoryRepository;
use super::service::CategoryService;

#[post("/v1/categories")]
async fn create_category(
    state: web::Data<AppState>,
    item: web::Json<CreateCategoryRequest>,
) -> impl Responder {
    let mut item: CreateCategoryRequest = item.into_inner();
    item.title = item.title.trim().to_string();

    if let Err(errors) = item.validate() {
        return api_validation_error(errors);
    }

    let user_id = ObjectId::new();
    let repository = CategoryRepository::new(&state.mongodb);
    let service = CategoryService::new(repository);
    match service
        .create_category(user_id, item.title.clone(), item.color)
        .await
    {
        Ok(id) => api_success("Category created successfully", Some(id.to_string())),
        Err(err) => api_unknown_error(err.to_string().as_str()),
    }
}

#[get("/v1/categories")]
async fn get_categories(state: web::Data<AppState>) -> impl Responder {
    let repository = CategoryRepository::new(&state.mongodb);
    let service = CategoryService::new(repository);

    match service.get_all_categories().await {
        Ok(categories) => api_success("Categories retrieved successfully", Some(categories)),
        Err(err) => api_unknown_error(err.to_string().as_str()),
    }
}

pub fn handles(cfg: &mut web::ServiceConfig) {
    cfg.service(create_category).service(get_categories);
}
