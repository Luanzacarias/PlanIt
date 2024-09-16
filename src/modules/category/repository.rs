use crate::category::models::Color;
use mongodb::bson::oid::ObjectId;
use mongodb::error::Error;
use mongodb::{bson::doc, Collection, Database};

use super::models::Category;

pub struct CategoryRepository {
    collection: Collection<Category>,
}

impl CategoryRepository {
    pub fn new(db: &Database) -> Self {
        let collection = db.collection("categories");
        CategoryRepository { collection }
    }

    pub async fn create_category(
        &self,
        new_category: Category,
    ) -> Result<mongodb::bson::oid::ObjectId, Error> {
        let result = self.collection.insert_one(new_category).await?;
        let id = result.inserted_id.as_object_id().unwrap();
        Ok(id)
    }

    pub async fn update_category(
        &self,
        id: ObjectId,
        title: String,
        color: Color,
    ) -> Result<(), Error> {
        let filter = doc! { "_id": id };
        let update = doc! { "$set": {
            "title": title,
            "color": color.as_str(),
        } };

        self.collection.update_one(filter, update).await?;
        Ok(())
    }

    pub async fn delete_category(&self, category_id: ObjectId) -> Result<(), Error> {
        let filter = doc! {"_id": category_id};
        self.collection.delete_one(filter).await?;
        Ok(())
    }

    pub async fn get_all_user_categories(
        &self,
        &user_id: &ObjectId,
    ) -> Result<Vec<Category>, Error> {
        let mut cursor = self
            .collection
            .find(doc! {
                "user_id": user_id
            })
            .await?;
        let mut categories: Vec<Category> = Vec::new();

        while cursor.advance().await? {
            categories.push(cursor.deserialize_current()?);
        }

        Ok(categories)
    }
    pub async fn get_category_by_title(
        &self,
        &user_id: &ObjectId,
        title: &str,
    ) -> Result<Option<Category>, Error> {
        let filter = doc! {"user_id": user_id, "title": title};
        self.collection.find_one(filter).await
    }

    pub async fn get_category_by_id(
        &self,
        user_id: &ObjectId,
        category_id: &ObjectId,
    ) -> Result<Option<Category>, Error> {
        let filter = doc! {
            "_id": category_id,
            "user_id": user_id
        };
        self.collection.find_one(filter).await
    }
}
