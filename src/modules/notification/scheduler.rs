use crate::modules::task::{models::Task, repository::TaskRepository};

use chrono::{Duration, Utc};
use log::{debug, error};
use tokio::{
    sync::Semaphore,
    time::{sleep, Duration as TokioDuration},
};

const MAX_NOTIFICATIONS: usize = 1;

pub async fn boot(task_repository: &TaskRepository) {
    let semaphore = Semaphore::new(MAX_NOTIFICATIONS);

    loop {
        debug!("Looping to check notifications");
        if let Err(e) = check_and_send_notifications(task_repository, &semaphore).await {
            error!("Error while checking notifications: {}", e);
        }

        sleep(TokioDuration::from_secs(60)).await;
    }
}

pub async fn check_and_send_notifications(
    repository: &TaskRepository,
    semaphore: &Semaphore,
) -> Result<(), Box<dyn std::error::Error>> {
    debug!("Checking for new notifications");
    let now = Utc::now() - Duration::hours(3); // TODO: Remove hardcoded timezone
    let upper_bound = now + chrono::Duration::seconds(60);
    let tasks = repository
        .get_all_not_sent_notifications(now, upper_bound)
        .await?;
    debug!("Found {} tasks to notify", tasks.len());

    for task in tasks {
        let permit = semaphore.acquire().await;
        match permit {
            Ok(_permit) => {
                if let Some(notification) = &task.notification {
                    if let Err(e) = process_notification(repository, &task).await {
                        error!(
                            "Error while processing notification for task {}: {}",
                            task.id.unwrap(),
                            e
                        );
                    }
                }
            }
            Err(e) => error!("Error while acquiring semaphore permit: {}", e),
        }
    }

    Ok(())
}

async fn process_notification(
    repository: &TaskRepository,
    task: &Task,
) -> Result<(), Box<dyn std::error::Error>> {
    repository
        .mark_notification_as_sent(&task.id.unwrap())
        .await?;
    // TODO: Send notification to user
    Ok(())
}
