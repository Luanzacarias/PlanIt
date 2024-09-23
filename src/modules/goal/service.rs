use mongodb::bson::oid::ObjectId;
use mongodb::error::Error;
use thiserror::Error;
use chrono::Utc;

use super::dto::{CreateGoalRequest, UpdateGoalRequest, GoalResponse};
use super::models::{Goal, Status};
use super::repository::GoalRepository;

#[derive(Error, Debug)]
pub enum GoalServiceError {
    #[error("Goal already exists")]
    GoalAlreadyExists,

    #[error("Database error occurred: {0}")]
    DatabaseError(#[from] Error),

    #[error("Goal not found")]
    GoalNotFound,
}

pub struct GoalService {
    repository: GoalRepository,
}

impl GoalService {
    pub fn new(repository: GoalRepository) -> Self {
        GoalService { repository }
    }

    pub async fn create_goal_for_user(
        &self,
        user_id: ObjectId,
        request: CreateGoalRequest,
    ) -> Result<ObjectId, GoalServiceError> {
        if let Some(_existing_goal) = self
            .repository
            .get_goal_by_title(&user_id, &request.title)
            .await?
        {
            return Err(GoalServiceError::GoalAlreadyExists);
        }

        let goal = Goal {
            id: None,
            user_id,
            title: request.title,
            description: request.description,
            end_date: request.end_date,
            category_id: request.category_id,
            priority: request.priority,
            status: Status::NotReached,
        };

        Ok(self.repository.create_goal(goal).await?)
    }

    pub async fn update_user_goal(
        &self,
        user_id: ObjectId,
        id: ObjectId,
        request: UpdateGoalRequest,
    ) -> Result<bool, GoalServiceError> {
        if self.repository.get_user_goal_by_id(user_id, id).await?.is_none() {
            return Err(GoalServiceError::GoalNotFound);
        }

        let result = self.repository.update_goal(
            id,
            request.title,
            request.description,
            request.end_date,
            request.priority,
            request.status,
            request.category_id,
        ).await?;
        Ok(result)
    }

    pub async fn delete_user_goal(
        &self,
        user_id: ObjectId,
        goal_id: ObjectId,
    ) -> Result<bool, GoalServiceError> {
        let result = self
            .repository
            .get_user_goal_by_id(user_id, goal_id)
            .await;
        if let Ok(Some(_goal)) = result {
            Ok(self.repository.delete_goal(goal_id).await?)
        } else {
            Err(GoalServiceError::GoalNotFound)
        }
    }

    pub async fn get_all_user_goals(
        &self,
        user_id: &ObjectId,
    ) -> Result<Vec<Goal>, Error> {
        let goals = self.repository.get_all_user_goals(user_id).await?;
        Ok(goals)
    }
}