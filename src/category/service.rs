use mongodb::bson::oid::ObjectId;
use mongodb::error::Error;

use super::models::{Category, Color};
use super::repository::CategoryRepository;

pub struct CategoryService {
    repository: CategoryRepository,
}

impl CategoryService {
    pub fn new(repository: CategoryRepository) -> Self {
        CategoryService { repository }
    }

    pub async fn create_category(
        &self,
        user_id: ObjectId,
        title: String,
        color: Color,
    ) -> Result<ObjectId, Error> {
        let new_category = Category {
            id: None,
            user_id,
            title,
            color,
        };

        // TODO: Validate if already exists a category with the same title for the same user

        self.repository.create_category(new_category).await
    }

    pub async fn get_all_categories(&self) -> Result<Vec<Category>, Error> {
        self.repository.get_all_categories().await
    }
}
