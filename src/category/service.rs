use crate::category::models::Category;
use crate::category::repository::CategoryRepository;
use mongodb::bson::oid::ObjectId;
use mongodb::error::Error;

use super::models::Color;

pub struct CategoryService {
    repository: CategoryRepository,
}

impl CategoryService {
    pub fn new(repository: CategoryRepository) -> Self {
        CategoryService { repository }
    }

    pub async fn create_category(
        &self,
        title: String,
        color: Color,
    ) -> Result<ObjectId, Error> {
        let new_category = Category {
            id: None,
            title,
            color,
        };
        self.repository.create_category(new_category).await
    }

    pub async fn get_all_categories(&self) -> Result<Vec<Category>, Error> {
        self.repository.get_all_categories().await
    }
}
