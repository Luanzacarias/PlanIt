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

    pub async fn create_goal(
        &self,
        request: CreateGoalRequest,
    ) -> Result<GoalResponse, GoalServiceError> {
        if let Some(_existing_goal) = self
            .repository
            .get_goal_by_title(&request.user_id, &request.title)
            .await?
        {
            return Err(GoalServiceError::GoalAlreadyExists);
        }

        let goal = Goal {
            id: None,
            user_id: request.user_id.clone(),
            title: request.title.clone(),
            description: request.description.clone(),
            end_date: request.end_date.unwrap_or_else(|| Utc::now()),
            priority: request.priority.clone(),
            status: Status::NotReached,
        };

        let result = self.repository.create_goal(goal).await?;
        Ok(GoalResponse {
            _id: result.to_hex(),
            title: request.title,
            description: request.description,
            end_date: request.end_date,
            priority: request.priority,
            status: Status::NotReached,
        })
    }

    pub async fn update_goal(
        &self,
        id: ObjectId,
        request: UpdateGoalRequest,
    ) -> Result<GoalResponse, GoalServiceError> {
        if self.repository.get_goal_by_id(&request.user_id, &id).await?.is_none() {
            return Err(GoalServiceError::GoalNotFound);
        }

        self.repository.update_goal(
            id,
            request.title.clone(),
            request.description.clone(),
            request.end_date.clone(),
            request.priority.clone(),
            request.status.clone(),
        ).await?;

        let updated_goal = self.repository.get_goal_by_id(&request.user_id, &id).await?.unwrap();
        Ok(GoalResponse {
            _id: updated_goal.id.unwrap().to_hex(),
            title: updated_goal.title,
            description: updated_goal.description,
            end_date: Some(updated_goal.end_date),
            priority: updated_goal.priority,
            status: updated_goal.status,
        })
    }

    pub async fn delete_user_goal(
        &self,
        goal_id: ObjectId,
        user_id: &ObjectId,
    ) -> Result<(), GoalServiceError> {
        let result = self
            .repository
            .get_goal_by_id(user_id, &goal_id)
            .await;
        if let Ok(Some(_goal)) = result {
            self.repository.delete_goal(goal_id).await?;
            Ok(())
        } else {
            Err(GoalServiceError::GoalNotFound)
        }
    }

    pub async fn get_all_user_goals(
        &self,
        user_id: &ObjectId,
    ) -> Result<Vec<GoalResponse>, Error> {
        let goals = self.repository.get_all_user_goals(user_id).await?;
        Ok(goals.into_iter().map(|goal| GoalResponse {
            _id: goal.id.unwrap().to_hex(),
            title: goal.title,
            description: goal.description,
            end_date: Some(goal.end_date),
            priority: goal.priority,
            status: goal.status,
        }).collect())
    }

    pub async fn get_all_goals(&self) -> Result<Vec<GoalResponse>, GoalServiceError> {
        let goals = self.repository.get_all_goals().await?;
        Ok(goals.into_iter().map(|goal| GoalResponse {
            _id: goal.id.unwrap().to_hex(),
            title: goal.title,
            description: goal.description,
            end_date: Some(goal.end_date),
            priority: goal.priority,
            status: goal.status,
        }).collect())
    }

    pub async fn get_goal_by_id(
        &self,
        user_id: &ObjectId,
        goal_id: &ObjectId,
    ) -> Result<Option<Goal>, GoalServiceError> {
        self.repository.get_goal_by_id(user_id, goal_id).await.map_err(GoalServiceError::DatabaseError)
    }
}