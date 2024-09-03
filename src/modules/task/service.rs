use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use mongodb::error::Error;

use thiserror::Error;

use super::models::{Task, Status};
use super::repository::TaskRepository;

#[derive(Error, Debug)]
pub enum TaskServiceError {
    #[error("Task already exists")]
    TaskAlreadyExists,

    #[error("Database error occurred: {0}")]
    DatabaseError(#[from] Error),
}

pub struct TaskService {
    repository: TaskRepository,
}

impl TaskService {
    pub fn new(repository: TaskRepository) -> Self {
        TaskService { repository }
    }

    pub async fn create_task_for_user(
        &self,
        title: String,
        description: String,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
        status: Status,
        &user_id: &ObjectId,
        &category_id: &ObjectId,
    ) -> Result<ObjectId, TaskServiceError> {
        if let Some(_existing_task) = self
            .repository
            .get_task_by_title(&user_id, &category_id, &title)
            .await?
        {
            return Err(TaskServiceError::TaskAlreadyExists);
        }
        let new_task = Task {
            id: None,
            title,
            description,
            start_date,
            end_date,
            status,
            user_id,
            category_id,
        };

        let result = self.repository.create_task(new_task).await?;
        Ok(result)
    }

    pub async fn get_all_user_tasks(
        &self,
        &user_id: &ObjectId,
    ) -> Result<Vec<Task>, Error> {
        self.repository.get_all_user_tasks(&user_id).await
    }
}
