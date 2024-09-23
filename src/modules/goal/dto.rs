use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::modules::category::dto::CategoryResponse;

use super::models::{Priority, Status};

#[derive(Debug, Validate, Serialize, Deserialize)]
pub struct CreateGoalRequest {
    pub title: String,
    pub description: String,
    pub category_id: Option<ObjectId>,
    pub end_date: Option<DateTime<Utc>>,
    pub priority: Priority,
}

#[derive(Debug, Validate, Serialize, Deserialize)]
pub struct UpdateGoalRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub category_id: Option<ObjectId>,
    pub end_date: Option<DateTime<Utc>>,
    pub priority: Option<Priority>,
    pub status: Option<Status>,
}

#[derive(Serialize)]
pub struct GoalResponse {
    pub _id: String,
    pub title: String,
    pub description: String,
    pub category: Option<CategoryResponse>,
    pub end_date: Option<DateTime<Utc>>,
    pub priority: Priority,
    pub status: Status,
}
