use crate::helpers::object_id_helper::{deserialize_option_object_id, serialize_option_object_id};
use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "EXECUTADA")]
    Executada,
    #[serde(rename = "PARCIALMENTE_EXECUTADA")]
    ParcialmenteExecutada,
    #[serde(rename = "ADIADA")]
    Adiada,
}

impl Status {
    pub fn as_str(&self) -> &str {
        match self {
            Status::Executada => "EXECUTADA",
            Status::ParcialmenteExecutada => "PARCIALMENTE_EXECUTADA",
            Status::Adiada => "ADIADA",
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    #[serde(
        rename = "_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_option_object_id",
        deserialize_with = "deserialize_option_object_id"
    )]
    pub id: Option<ObjectId>,
    pub title: String,
    pub description: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub status: Status,
    pub user_id: ObjectId,
    pub category_id: ObjectId,
    pub notification: Option<crate::modules::notification::models::Notification>,
}

#[derive(Serialize, Deserialize)]
pub struct TaskByCategoryAndStatus {
    pub category: String,
    pub status: String,
    pub count: i32,
}

#[derive(Serialize, Deserialize)]
pub struct TaskStatsByCategory {
    pub category: String,
    pub completed_count: i32,
    pub postponed_count: i32,
    pub partially_completed_count: i32,
}
