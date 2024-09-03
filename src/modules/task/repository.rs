use mongodb::bson::oid::ObjectId;
use mongodb::error::Error;
use mongodb::{bson::doc, Collection, Database};

use super::models::Task;

pub struct TaskRepository {
    collection: Collection<Task>,
}

impl TaskRepository {
    pub fn new(db: &Database) -> Self {
        let collection = db.collection("tasks");
        TaskRepository { collection }
    }

    pub async fn create_task(
        &self,
        new_task: Task,
    ) -> Result<mongodb::bson::oid::ObjectId, Error> {
        let result = self.collection.insert_one(new_task).await?;
        let id = result.inserted_id.as_object_id().unwrap();
        Ok(id)
    }

    pub async fn get_all_user_tasks(
        &self,
        &user_id: &ObjectId,
    ) -> Result<Vec<Task>, Error> {
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

    pub async fn get_task_by_title(
        &self,
        &user_id: &ObjectId,
        &category_id: &ObjectId,
        title: &str,
    ) -> Result<Option<Task>, Error> {
        let filter = doc! {"user_id": user_id, "category_id": category_id, "title": title};
        self.collection.find_one(filter).await
    }

}
