use mongodb::bson::{oid::ObjectId, Bson};
use serde::{Deserialize, Deserializer, Serializer};

pub fn serialize_option_object_id<S>(
    id: &Option<ObjectId>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match id {
        Some(oid) => serializer.serialize_some(&oid.to_string()),
        None => serializer.serialize_none(),
    }
}

pub fn deserialize_option_object_id<'de, D>(deserializer: D) -> Result<Option<ObjectId>, D::Error>
where
    D: Deserializer<'de>,
{
    let bson: Bson = Bson::deserialize(deserializer)?;
    match bson {
        Bson::ObjectId(oid) => Ok(Some(oid)),
        Bson::Document(mut doc) => {
            if let Some(Bson::String(oid_str)) = doc.remove("$oid") {
                ObjectId::parse_str(&oid_str).map(Some).map_err(serde::de::Error::custom)
            } else {
                Err(serde::de::Error::custom("expected $oid field"))
            }
        }
        Bson::String(oid_str) => ObjectId::parse_str(&oid_str)
            .map(Some)
            .map_err(serde::de::Error::custom),
        _ => Err(serde::de::Error::custom("expected ObjectId")),
    }
}

pub fn serialize_object_id<S>(id: &ObjectId, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&id.to_string())
}

pub fn deserialize_object_id<'de, D>(deserializer: D) -> Result<ObjectId, D::Error>
where
    D: Deserializer<'de>,
{
    let bson: Bson = Bson::deserialize(deserializer)?;
    match bson {
        Bson::ObjectId(oid) => Ok(oid),
        Bson::Document(mut doc) => {
            if let Some(Bson::String(oid_str)) = doc.remove("$oid") {
                ObjectId::parse_str(&oid_str).map_err(serde::de::Error::custom)
            } else {
                Err(serde::de::Error::custom("expected $oid field"))
            }
        }
        Bson::String(oid_str) => ObjectId::parse_str(&oid_str).map_err(serde::de::Error::custom),
        _ => Err(serde::de::Error::custom("expected ObjectId")),
    }
}
