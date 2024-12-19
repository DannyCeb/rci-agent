use std::sync::Arc;

use tokio::sync::Mutex;

use crate::utils::stared_data::{SharedData, TaskStatus};

pub async fn execute_task(task_id: String, shared_data: Arc<Mutex<SharedData>>) {
    let task = {
        shared_data
            .clone()
            .lock()
            .await
            .get_task_clone_by_id(&task_id)
    };

    match task {
        Some(mut task_wrapper) => {
            task_wrapper.set_status(TaskStatus::Active);
        }
        None => {
            return;
        }
    }
}
