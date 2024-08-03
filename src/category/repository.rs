use crate::category::models::Category;
use mongodb::error::Error;
use mongodb::{bson::doc, Collection, Database};

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

    pub async fn get_all_categories(&self) -> Result<Vec<Category>, Error> {
        let mut cursor = self.collection.find(doc! {}).await?;
        let mut categories: Vec<Category> = Vec::new();

        while cursor.advance().await? {
            categories.push(cursor.deserialize_current()?);
        }

        Ok(categories)
    }
}
