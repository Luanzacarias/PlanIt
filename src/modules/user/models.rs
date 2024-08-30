use crate::helpers::object_id_helper::{deserialize_option_object_id, serialize_option_object_id};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(
        rename = "_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_option_object_id",
        deserialize_with = "deserialize_option_object_id"
    )]
    pub id: Option<ObjectId>,
    pub name: String,
    pub email: String, // TODO: create a custom email type
    pub password: String, // TODO: create a custom password type
    pub phone: String, // TODO: create a custom phone type
}
