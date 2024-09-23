use mongodb::bson::{doc, oid::ObjectId};
use mongodb::error::Error;
use mongodb::Collection;
use chrono::{DateTime, SecondsFormat, Utc};

use super::models::{Goal, Priority, Status};

pub struct GoalRepository {
    collection: Collection<Goal>,
}

impl GoalRepository {
    pub fn new(db: &mongodb::Database) -> Self {
        let collection = db.collection("goals");
        GoalRepository { collection }
    }

    pub async fn create_goal(&self, new_goal: Goal) -> Result<ObjectId, Error> {
        let result = self.collection.insert_one(new_goal).await?;
        let id = result.inserted_id.as_object_id().unwrap();
        Ok(id)
    }

    pub async fn update_goal(
        &self,
        id: ObjectId,
        title: Option<String>,
        description: Option<String>,
        end_date: Option<DateTime<Utc>>,
        priority: Option<Priority>,
        status: Option<Status>,
    ) -> Result<bool, Error> {
        let filter = doc! { "_id": id };
        let mut update_doc = doc! {};

        if let Some(title) = title {
            update_doc.insert("title", title);
        }
        if let Some(description) = description {
            update_doc.insert("description", description);
        }
        if let Some(end_date) = end_date {
            update_doc.insert("end_date", end_date.to_rfc3339_opts(SecondsFormat::Secs, true));
        }
        if let Some(priority) = priority {
            update_doc.insert("priority", priority.as_str());
        }
        if let Some(status) = status {
            update_doc.insert("status", status.as_str());
        }

        let update = doc! { "$set": update_doc };

        let result = self.collection.update_one(filter, update).await?;

        Ok(result.modified_count > 0)
    }

    pub async fn delete_goal(&self, goal_id: ObjectId) -> Result<bool, Error> {
        let query = doc! { "_id": goal_id };

        let result = self.collection.delete_one(query).await?;

        Ok(result.deleted_count > 0)
    }

    pub async fn get_all_user_goals(&self, user_id: &ObjectId) -> Result<Vec<Goal>, Error> {
        let mut cursor = self.collection.find(doc! { "user_id": user_id }).await?;
        let mut goals: Vec<Goal> = Vec::new();

        while cursor.advance().await? {
            goals.push(cursor.deserialize_current()?);
        }

        Ok(goals)
    }

    pub async fn get_goal_by_id(&self, goal_id: &ObjectId) -> Result<Option<Goal>, Error> {
        let filter = doc! { "_id": goal_id };
        self.collection.find_one(filter).await
    }

    pub async fn get_goal_by_title(&self, user_id: &ObjectId, title: &str) -> Result<Option<Goal>, Error> {
        let filter = doc! { "user_id": user_id, "title": title };
        self.collection.find_one(filter).await
    }

    pub async fn get_all_goals(&self) -> Result<Vec<Goal>, Error> {
        let mut cursor = self.collection.find(doc! {}).await?;
        let mut goals: Vec<Goal> = Vec::new();
        while cursor.advance().await? {
            goals.push(cursor.deserialize_current()?);
        }

        Ok(goals)
    }
}