use mongodb::error::Error;
use mongodb::{bson::doc, Collection, Database};

use super::models::User;

pub struct UserRepository {
    collection: Collection<User>,
}

impl UserRepository {
    pub fn new(db: &Database) -> Self {
        let collection = db.collection("users");
        UserRepository { collection }
    }

    pub async fn create_user(&self, new_user: User) -> Result<mongodb::bson::oid::ObjectId, Error> {
        let result = self.collection.insert_one(new_user).await?;
        Ok(result.inserted_id.as_object_id().unwrap())
    }

    pub async fn find_user_by_email(&self, email: &str) -> Result<Option<User>, Error> {
        let user = self.collection.find_one(doc! { "email": email }).await?;
        Ok(user)
    }
}
