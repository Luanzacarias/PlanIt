use crate::modules::{category::dto::CategoryResponse, notification::models::TimeUnit};

use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::models::Status;

#[derive(Deserialize, Validate)]
pub struct CreateTaskRequest {
    #[validate(length(min = 1, max = 30))]
    pub title: String,
    #[validate(length(min = 1, max = 100))]
    pub description: String,
    #[allow(dead_code)]
    pub start_date: DateTime<Utc>,
    #[allow(dead_code)]
    pub end_date: DateTime<Utc>,
    #[allow(dead_code)]
    pub status: Status,
    #[allow(dead_code)]
    pub category_id: ObjectId,
    pub notification_time_unit: Option<TimeUnit>,
    pub notification_time_value: Option<u16>,
}

#[derive(Deserialize, Validate)]
pub struct UpdateTaskRequest {
    #[validate(length(min = 1, max = 30))]
    pub title: Option<String>,
    #[validate(length(min = 1, max = 100))]
    pub description: Option<String>,
    #[allow(dead_code)]
    pub start_date: Option<DateTime<Utc>>,
    #[allow(dead_code)]
    pub end_date: Option<DateTime<Utc>>,
    #[allow(dead_code)]
    pub status: Option<Status>,
    #[allow(dead_code)]
    pub category_id: Option<ObjectId>,
    pub notification_time_unit: Option<Option<TimeUnit>>,
    pub notification_time_value: Option<Option<u16>>,
}

#[derive(Serialize)]
pub struct TaskResponse {
    pub _id: String,
    pub title: String,
    pub description: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub status: Status,
    pub category: Option<CategoryResponse>,
    pub notification_time_unit: Option<TimeUnit>,
    pub notification_time_value: Option<u16>,
}
