use mongodb::bson::oid::ObjectId;
use thiserror::Error;

use super::dto::UserSignUpRequest;
use super::models::User;
use super::repository::UserRepository;


#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("User with this email already exists")]
    UserAlreadyExists,
    #[error("Database error: {0}")]
    DatabaseError(#[from] mongodb::error::Error),
}

pub struct UserService {
    repository: UserRepository,
}

impl UserService {
    pub fn new(repository: UserRepository) -> Self {
        UserService { repository }
    }

    pub async fn create_user(&self, data: UserSignUpRequest) -> Result<ObjectId, ServiceError> {
        if (self.repository.find_user_by_email(&data.email).await?).is_some() {
            return Err(ServiceError::UserAlreadyExists);
        }

        let new_user = User {
            id: None,
            name: data.name,
            email: data.email,
            password: data.password,
            phone: data.phone,
        };
        self.repository
            .create_user(new_user)
            .await
            .map_err(ServiceError::from)
    }
}
