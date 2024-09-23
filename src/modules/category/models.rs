use crate::helpers::object_id_helper::{deserialize_option_object_id, serialize_option_object_id};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Color {
    #[serde(rename = "ORANGE")]
    Orange,
    #[serde(rename = "YELLOW")]
    Yellow,
    #[serde(rename = "GREEN")]
    Green,
    #[serde(rename = "RED")]
    Red,
    #[serde(rename = "PURPLE")]
    Purple,
}

impl Color {
    pub fn as_str(&self) -> &str {
        match self {
            Color::Orange => "ORANGE",
            Color::Yellow => "YELLOW",
            Color::Green => "GREEN",
            Color::Red => "RED",
            Color::Purple => "PURPLE",
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    #[serde(
        rename = "_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_option_object_id",
        deserialize_with = "deserialize_option_object_id"
    )]
    pub id: Option<ObjectId>,
    pub user_id: ObjectId,
    pub title: String,
    pub color: Color,
}
