use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::models::{Priority, Status};

#[derive(Debug, Validate, Serialize, Deserialize)]
pub struct CreateGoalRequest {
    pub title: String,
    pub description: String,
    pub end_date: Option<DateTime<Utc>>, // Tornando end_date opcional
    pub priority: Priority,
    pub user_id: ObjectId,
}

#[derive(Debug, Validate, Serialize, Deserialize)]
pub struct UpdateGoalRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub end_date: Option<DateTime<Utc>>, // Tornando end_date opcional
    pub priority: Option<Priority>,
    pub status: Option<Status>,
    pub user_id: ObjectId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoalResponse {
    pub _id: String,
    pub title: String,
    pub description: String,
    pub end_date: Option<DateTime<Utc>>, // Tornando end_date opcional
    pub priority: Priority,
    pub status: Status,
}
