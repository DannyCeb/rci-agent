use std::{sync::Arc, time::Duration};
use tokio::{sync::Mutex, time::sleep};

use crate::utils::stared_data::SharedData;

use super::executor::execute_task;

pub async fn executor_background(shared_data: Arc<Mutex<SharedData>>) {
    loop {
        {
            let guard = shared_data.lock().await;
            let h_tasks = guard.get_htasks_by_ref();

            for (task_id, task_wrapper) in h_tasks {
                if task_wrapper.is_done() {
                    continue;
                }

                if task_wrapper.is_active() {
                    continue;
                }

                let task_id = task_id.clone();
                let shared_data = shared_data.clone();

                tokio::spawn(async move {
                    let _ = execute_task(task_id, shared_data).await;
                });
            }
        }

        sleep(Duration::from_secs(1)).await;
    }
}
