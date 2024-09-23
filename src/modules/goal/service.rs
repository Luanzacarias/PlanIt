use mongodb::bson::oid::ObjectId;
use mongodb::error::Error;
use thiserror::Error;

use super::dto::{CreateGoalRequest, UpdateGoalRequest, GoalResponse};
use super::models::Goal;
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
        user_id: &ObjectId,
        request: CreateGoalRequest,
    ) -> Result<GoalResponse, GoalServiceError> {
        if let Some(_existing_goal) = self
            .repository
            .get_goal_by_title(user_id, &request.title)
            .await?
        {
            return Err(GoalServiceError::GoalAlreadyExists);
        }

        let goal = Goal {
            id: None,
            user_id: user_id.clone(),
            title: request.title.clone(),
            description: request.description.clone(),
            start_date: request.start_date,
            end_date: request.end_date,
            priority: request.priority.clone(),
        };

        let result = self.repository.create_goal(goal).await?;
        Ok(GoalResponse {
            _id: result.to_hex(),
            title: request.title,
            description: request.description,
            start_date: request.start_date,
            end_date: request.end_date,
            priority: request.priority,
        })
    }

    pub async fn update_goal(
        &self,
        user_id: &ObjectId,
        id: ObjectId,
        request: UpdateGoalRequest,
    ) -> Result<GoalResponse, GoalServiceError> {
        if self.repository.get_goal_by_id(user_id, &id).await?.is_none() {
            return Err(GoalServiceError::GoalNotFound);
        }

        self.repository.update_goal(
            id,
            request.title.clone(),
            request.description.clone(),
            request.start_date,
            request.end_date,
            request.priority.clone(),
        ).await?;

        let updated_goal = self.repository.get_goal_by_id(user_id, &id).await?.unwrap();
        Ok(GoalResponse {
            _id: updated_goal.id.unwrap().to_hex(),
            title: updated_goal.title,
            description: updated_goal.description,
            start_date: updated_goal.start_date,
            end_date: updated_goal.end_date,
            priority: updated_goal.priority,
        })
    }

    pub async fn delete_goal(
        &self,
        user_id: &ObjectId,
        id: ObjectId,
    ) -> Result<(), GoalServiceError> {
        if self.repository.get_goal_by_id(user_id, &id).await?.is_none() {
            return Err(GoalServiceError::GoalNotFound);
        }

        self.repository.delete_goal(id).await?;
        Ok(())
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
            start_date: goal.start_date,
            end_date: goal.end_date,
            priority: goal.priority,
        }).collect())
    }

    pub async fn get_goal_by_id(
        &self,
        user_id: &ObjectId,
        goal_id: &ObjectId,
    ) -> Result<Option<Goal>, GoalServiceError> {
        self.repository.get_goal_by_id(user_id, goal_id).await.map_err(GoalServiceError::DatabaseError)
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
}
