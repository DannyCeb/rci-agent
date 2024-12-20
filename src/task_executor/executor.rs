use std::sync::Arc;

use tokio::sync::Mutex;

use crate::utils::{
    error::RciError,
    stared_data::{SharedData, TaskStatus},
};

pub async fn execute_task(
    task_id: String,
    shared_data: Arc<Mutex<SharedData>>,
) -> Result<(), RciError> {
    let mut guard = shared_data.lock().await;

    let task_wrapper = guard.get_htasks_by_mut_ref().get_mut(&task_id).unwrap();

    task_wrapper.set_status(TaskStatus::Active);

    task_wrapper.execute().await?;

    Ok(())
}
