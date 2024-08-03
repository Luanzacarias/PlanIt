use mongodb::bson::{oid::ObjectId, Bson};
use serde::{Deserialize, Deserializer, Serializer};

pub fn serialize_object_id<S>(id: &Option<ObjectId>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match id {
        Some(oid) => serializer.serialize_some(&oid.to_string()),
        None => serializer.serialize_none(),
    }
}

pub fn deserialize_object_id<'de, D>(deserializer: D) -> Result<Option<ObjectId>, D::Error>
where
    D: Deserializer<'de>,
{
    let bson: Bson = Bson::deserialize(deserializer)?;
    match bson {
        Bson::ObjectId(oid) => Ok(Some(oid)),
        Bson::Document(mut doc) => {
            if let Some(Bson::String(oid_str)) = doc.remove("$oid") {
                Ok(Some(ObjectId::parse_str(&oid_str).unwrap()))
            } else {
                Err(serde::de::Error::custom("expected $oid field"))
            }
        }
        _ => Err(serde::de::Error::custom("expected ObjectId")),
    }
}
