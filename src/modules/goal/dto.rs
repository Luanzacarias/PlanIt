use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::models::Priority;

#[derive(Deserialize, Validate)]
pub struct CreateGoalRequest {
    #[validate(length(min = 1, max = 100))]
    pub title: String,
    #[validate(length(min = 1, max = 500))]
    pub description: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub priority: Priority,
}

#[derive(Deserialize, Validate)]
pub struct UpdateGoalRequest {
    #[validate(length(min = 1, max = 100))]
    pub title: Option<String>,
    #[validate(length(min = 1, max = 500))]
    pub description: Option<String>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub priority: Option<Priority>,
}

#[derive(Serialize)]
pub struct GoalResponse {
    pub _id: String,
    pub title: String,
    pub description: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub priority: Priority,
}
