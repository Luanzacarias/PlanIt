use crate::modules::notification::models::Notification;
use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use mongodb::bson::Bson;
use mongodb::error::Error;
use mongodb::{bson::doc, Collection, Database};

use crate::modules::category;

use super::models::{Status, Task, TaskByCategoryAndStatus};

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

    pub async fn get_task_by_id(&self, &task_id: &ObjectId) -> Result<Option<Task>, Error> {
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


    pub async fn count_tasks_by_status(
        &self,
        user_id: &mongodb::bson::oid::ObjectId,
    ) -> Result<Vec<TaskByCategoryAndStatus>, Error> {
        let pipeline = vec![
            doc! {
                "$match": {
                    "user_id": user_id,
                }
            },
            doc! {
                "$lookup": {
                    "from": "categories",
                    "localField": "category_id",
                    "foreignField": "_id",
                    "as": "category"
                }
            },
            doc! {
                "$unwind": {
                    "path": "$category"
                }
            },
            doc! {
                "$group": {
                    "_id": {
                        "category": "$category.title",
                        "status": "$status"
                    },
                    "count": { "$sum": 1 }
                }
            },
            doc! {
                "$project": {
                    "category": "$_id.category",
                    "status": "$_id.status",
                    "count": 1
                }
            },
        ];

        let mut cursor = self.collection.aggregate(pipeline).await?;
        let mut result: Vec<TaskByCategoryAndStatus> = Vec::new();

        while cursor.advance().await? {
            let doc = cursor.deserialize_current()?;

            if let (Some(category), Some(status), Some(count)) = (
                doc.get_str("category").ok(),
                doc.get_str("status").ok(),
                doc.get_i32("count").ok(),
            ) {
                result.push(TaskByCategoryAndStatus {
                    category: category.to_string(),
                    status: status.to_string(),
                    count,
                });
            }
        }

        Ok(result)
    }

    pub async fn get_all_not_sent_notifications(&self, greater_than: DateTime<Utc>, last_than_or_equals: DateTime<Utc>) -> Result<Vec<Task>, Error> {
        let filter = doc! {
            "notification.scheduled_time": { "$gte": greater_than.to_rfc3339(), "$lte": last_than_or_equals.to_rfc3339() },
            "notification.sent": false
        };

        let mut cursor = self.collection.find(filter).await?;
        let mut tasks = Vec::new();
        while cursor.advance().await? {
            tasks.push(cursor.deserialize_current()?);
        }

        Ok(tasks)
    }

    pub async fn mark_notification_as_sent(&self, task_id: &ObjectId) -> Result<bool, Error> {
        let filter = doc! { "_id": task_id, "notification.sent": false };
        let update = doc! { "$set": { "notification.sent": true } };
        let result = self.collection.update_one(filter, update).await?;

        Ok(result.modified_count > 0)
    }

    pub async fn get_all_with_notifications(&self, user_id: &ObjectId) -> Result<Vec<Task>, Error> {
        let filter = doc! {
            "user_id": user_id,
            "notification": { "$ne": null }
        };

        let mut cursor = self.collection.find(filter).await?;
        let mut notifications = Vec::new();
        while cursor.advance().await? {
            notifications.push(cursor.deserialize_current()?);
        }

        Ok(notifications)
    }
}
