use crate::helpers::object_id_helper::{deserialize_option_object_id, serialize_option_object_id};
use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Priority {
    #[serde(rename = "HIGH")]
    High,
    #[serde(rename = "MEDIUM")]
    Medium,
    #[serde(rename = "LOW")]
    Low,
}

impl Priority {
    pub fn as_str(&self) -> &str {
        match self {
            Priority::High => "HIGH",
            Priority::Medium => "MEDIUM",
            Priority::Low => "LOW",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "NOT_REACHED")]
    NotReached,
    #[serde(rename = "PARTIALLY_REACHED")]
    PartiallyReached,
    #[serde(rename = "REACHED")]
    Reached,
}

impl Status {
    pub fn as_str(&self) -> &str {
        match self {
            Status::NotReached => "NOT_REACHED",
            Status::PartiallyReached => "PARTIALLY_REACHED",
            Status::Reached => "REACHED",
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Goal {
    #[serde(
        rename = "_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_option_object_id",
        deserialize_with = "deserialize_option_object_id"
    )]
    pub id: Option<ObjectId>,
    pub title: String,
    pub description: String,
    pub category_id: Option<ObjectId>,
    pub end_date: Option<DateTime<Utc>>,
    pub priority: Priority,
    pub status: Status,
    pub user_id: ObjectId,
}