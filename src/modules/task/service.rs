use crate::modules::notification::models::{Notification, TimeUnit};

use std::collections::HashMap;
use chrono::Duration;
use mongodb::bson::oid::ObjectId;
use mongodb::error::Error;
use thiserror::Error;

use super::dto::{CreateTaskRequest, UpdateTaskRequest};
use super::models::{Task, TaskStatsByCategory};
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
        &user_id: &ObjectId,
        task_data: CreateTaskRequest,
    ) -> Result<ObjectId, TaskServiceError> {
        if let Some(_existing_task) = self
            .repository
            .get_task_by_title(&user_id, &task_data.title)
            .await?
        {
            return Err(TaskServiceError::TaskAlreadyExists);
        }

        let notification: Option<Notification> = {
            if task_data.notification_time_unit.is_none() || task_data.notification_time_value.is_none()
            {
                None
            } else {
                let time_value = task_data.notification_time_value.unwrap() as i64;
                Some(Notification {
                    id: ObjectId::new(),
                    sent: false,
                    scheduled_time: match task_data.notification_time_unit {
                        Some(TimeUnit::Minute) => task_data.start_date - Duration::minutes(time_value),
                        Some(TimeUnit::Hour) => task_data.start_date - Duration::hours(time_value),
                        None => task_data.start_date,
                    },
                    time_unit: task_data.notification_time_unit.unwrap(),
                    time_value: task_data.notification_time_value.unwrap(),
                    viewed: false,
                })
            }
        };

        let new_task = Task {
            id: None,
            title: task_data.title,
            description: task_data.description,
            start_date: task_data.start_date,
            end_date: task_data.end_date,
            status: task_data.status,
            user_id,
            category_id: task_data.category_id,
            notification,
        };

        let result = self.repository.create_task(new_task).await?;
        Ok(result)
    }

    pub async fn update_user_task(
        &self,
        &user_id: &ObjectId,
        task_id: &ObjectId,
        task_data: UpdateTaskRequest,
    ) -> Result<bool, TaskServiceError> {
        let old_data = self.repository.get_task_by_id(task_id).await?;
        if old_data.is_none() {
            return Err(TaskServiceError::TaskNotFound);
        }
    
        if let Some(title) = &task_data.title {
            if let Some(existing_task) = self
                .repository
                .get_task_by_title(&user_id, title)
                .await?
            {
                if existing_task.id.as_ref() != Some(task_id) {
                    return Err(TaskServiceError::TaskAlreadyExists);
                }
            }
        }

        let notification = match (task_data.notification_time_unit, task_data.notification_time_value) {
            (Some(Some(time_unit)), Some(Some(time_value))) => { // existi and has value
                let time_value = time_value as i64;
                let start_date = task_data.start_date.or(Some(old_data.unwrap().start_date)).unwrap();
                let scheduled_time = match time_unit {
                    TimeUnit::Minute => start_date - Duration::minutes(time_value),
                    TimeUnit::Hour => start_date - Duration::hours(time_value),
                };
                Some(Some(Notification {
                    id: ObjectId::new(),
                    time_unit,
                    time_value: time_value as u16,
                    scheduled_time,
                    sent: false,
                    viewed: false,
                }))
            },
            (Some(None), Some(None)) => Some(None), // Remove notification
            _ => None,
        };
    
        let result = self
            .repository
            .update_task(
                task_id,
                task_data.title,
                task_data.description,
                task_data.start_date,
                task_data.end_date,
                task_data.status,
                task_data.category_id,
                notification,
            )
            .await?;
    
        Ok(result)
    }
    
    pub async fn delete_user_task(&self, task_id: &ObjectId) -> Result<bool, TaskServiceError> {
        if self.repository.get_task_by_id(task_id).await?.is_none() {
            return Err(TaskServiceError::TaskNotFound);
        }

        let result = self.repository.delete_task(task_id).await?;

        Ok(result)
    }

    pub async fn get_all_user_tasks(&self, &user_id: &ObjectId) -> Result<Vec<Task>, Error> {
        self.repository.get_all_user_tasks(&user_id).await
    }

    pub async fn count_tasks_by_category_and_status(
        &self,
        user_id: &ObjectId,
    ) -> Result<Vec<TaskStatsByCategory>, Error> {
        let task_stats = self.repository.count_tasks_by_status(user_id).await?;

        let mut category_map: HashMap<String, TaskStatsByCategory> = HashMap::new();

        for task in task_stats {
            let entry = category_map
                .entry(task.category.clone())
                .or_insert(TaskStatsByCategory {
                    category: task.category.clone(),
                    completed_count: 0,
                    postponed_count: 0,
                    partially_completed_count: 0,
                });

            match task.status.as_str() {
                "EXECUTADA" => entry.completed_count += task.count,
                "ADIADA" => entry.postponed_count += task.count,
                "PARCIALMENTE_EXECUTADA" => entry.partially_completed_count += task.count,
                _ => (),
            }
        }

        let result: Vec<TaskStatsByCategory> = category_map.into_iter().map(|(_, v)| v).collect();

        Ok(result)
    }
}
