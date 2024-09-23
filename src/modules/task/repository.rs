use crate::modules::notification::models::Notification;

use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use mongodb::bson::Bson;
use mongodb::error::Error;
use mongodb::{bson::doc, Collection, Database};

use super::models::{Status, Task};

pub struct TaskRepository {
    collection: Collection<Task>,
}

impl TaskRepository {
    pub fn new(db: &Database) -> Self {
        let collection = db.collection("tasks");
        TaskRepository { collection }
    }

    pub async fn create_task(&self, new_task: Task) -> Result<mongodb::bson::oid::ObjectId, Error> {
        let result = self.collection.insert_one(new_task).await?;
        let id = result.inserted_id.as_object_id().unwrap();
        Ok(id)
    }

    pub async fn update_task(
        &self,
        task_id: &ObjectId,
        title: Option<String>,
        description: Option<String>,
        start_date: Option<DateTime<Utc>>,
        end_date: Option<DateTime<Utc>>,
        status: Option<Status>,
        category_id: Option<ObjectId>,
        notification: Option<Option<Notification>>,
    ) -> Result<bool, Error> {
        let filter = doc! { "_id": task_id };
    
        let mut update_doc = doc! {};
        if let Some(title) = title {
            update_doc.insert("title", title);
        }
        if let Some(description) = description {
            update_doc.insert("description", description);
        }
        if let Some(start_date) = start_date {
            update_doc.insert("start_date", start_date.to_rfc3339());
        }
        if let Some(end_date) = end_date {
            update_doc.insert("end_date", end_date.to_rfc3339());
        }
        if let Some(status) = status {
            update_doc.insert("status", Bson::String(status.as_str().to_string()));
        }
        if let Some(category_id) = category_id {
            update_doc.insert("category_id", category_id);
        }
        if let Some(notification) = notification {
            if notification.is_some() {
                let notification = notification.unwrap();
                let notification_doc = doc! {
                    "_id": notification.id,
                    "time_unit": notification.time_unit.as_str().to_string(),
                    "time_value": notification.time_value as i64,
                    "scheduled_time": notification.scheduled_time.to_rfc3339(),
                    "sent": notification.sent,
                };
                update_doc.insert("notification", notification_doc);
            } else {
                update_doc.insert("notification", Bson::Null);
            }
        }
    
        let update = doc! { "$set": update_doc };
        let result = self.collection.update_one(filter, update).await?;
    
        Ok(result.modified_count > 0)
    }
    
    pub async fn delete_task(&self, task_id: &ObjectId) -> Result<bool, Error> {
        let query = doc! {"_id": task_id};

        let result = self.collection.delete_one(query).await?;

        Ok(result.deleted_count > 0)
    }

    pub async fn get_all_user_tasks(&self, &user_id: &ObjectId) -> Result<Vec<Task>, Error> {
        let mut cursor = self
            .collection
            .find(doc! {
                "user_id": user_id
            })
            .await?;
        let mut tasks: Vec<Task> = Vec::new();

        while cursor.advance().await? {
            tasks.push(cursor.deserialize_current()?);
        }

        Ok(tasks)
    }

    pub async fn get_task_by_id(
        &self,
        &task_id: &ObjectId,
    ) -> Result<Option<Task>, Error> {
        let filter = doc! {"_id": task_id};
        self.collection.find_one(filter).await
    }

    pub async fn get_task_by_title(
        &self,
        &user_id: &ObjectId,
        title: &str,
    ) -> Result<Option<Task>, Error> {
        let filter = doc! {"user_id": user_id, "title": title};
        self.collection.find_one(filter).await
    }

}
