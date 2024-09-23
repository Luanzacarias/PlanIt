use crate::helpers::object_id_helper::{deserialize_object_id, serialize_object_id};
use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TimeUnit {
    #[serde(rename = "MINUTE")]
    Minute,
    #[serde(rename = "HOUR")]
    Hour,
}

impl TimeUnit {
    pub fn as_str(&self) -> &str {
        match self {
            TimeUnit::Minute => "MINUTE",
            TimeUnit::Hour => "HOUR",
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Notification {
    #[serde(
        rename = "_id",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub id: ObjectId,
    pub time_unit: TimeUnit,
    pub time_value: u16,
    pub scheduled_time: DateTime<Utc>,
    pub sent: bool,
}
