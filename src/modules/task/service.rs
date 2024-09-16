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

    #[error("Task not found")]
    TaskNotFound,

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

    pub async fn update_user_task(
        &self,
        task_id: &ObjectId,
        title: String,
        description: Option<String>,
        start_date: Option<DateTime<Utc>>,
        end_date: Option<DateTime<Utc>>,
        status: Option<Status>,
        user_id: &ObjectId,
        category_id: Option<ObjectId>,
    ) -> Result<bool, TaskServiceError> {
        if self.repository.get_task_by_id(task_id).await?.is_none() {
            return Err(TaskServiceError::TaskNotFound);
        }
        
        if let Some(_existing_task) = self
            .repository
            .get_task_by_title(user_id, &category_id.unwrap(), &title)
            .await?
        {
            if let Some(existing_task_id) = _existing_task.id {
                if existing_task_id != *task_id {
                    return Err(TaskServiceError::TaskAlreadyExists);
                }
            }
        }
    
        let result = self.repository.update_task(
            task_id,
            title,
            description,
            start_date,
            end_date,
            status,
            category_id,
        ).await?;
    
        Ok(result)
    }

    pub async fn delete_user_task(
        &self,
        task_id: &ObjectId,
    ) -> Result<bool, TaskServiceError> {
        if self.repository.get_task_by_id(task_id).await?.is_none() {
            return Err(TaskServiceError::TaskNotFound);
        }

        let result = self.repository.delete_task(task_id).await?;
    
        Ok(result)
    }

    pub async fn get_all_user_tasks(
        &self,
        &user_id: &ObjectId,
    ) -> Result<Vec<Task>, Error> {
        self.repository.get_all_user_tasks(&user_id).await
    }
}
