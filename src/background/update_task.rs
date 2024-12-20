use std::{sync::Arc, time::Duration};

use serde::{Deserialize, Serialize};
use tokio::{sync::Mutex, time::sleep};

use crate::utils::{
    error::RciError,
    stared_data::{SharedData, TaskStatus},
};

#[derive(Debug, Serialize, Deserialize)]
struct TaskUpdate {
    id: String,
    update_status: TaskStatus,
    result_message: Option<String>,
}

pub async fn update_tasks_on_server(shared_data: Arc<Mutex<SharedData>>) -> Result<(), RciError> {
    sleep(Duration::from_secs(2)).await;
    loop {
        let mut v_done_tasks_ids = Vec::<String>::new();
        {
            let mut guard = shared_data.lock().await;

            let h_tasks = guard.get_htasks_by_ref();
            if !h_tasks.is_empty() {
                let client = guard.get_http_client_by_ref();
                let token = guard.get_token_value_by_ref().clone();
                for (task_id, task_wrapper) in h_tasks {
                    let task = if task_wrapper.is_done() {
                        v_done_tasks_ids.push(task_id.clone());
                        TaskUpdate {
                            id: task_id.clone(),
                            update_status: TaskStatus::Done,
                            result_message: task_wrapper.get_result(),
                        }
                    } else if task_wrapper.is_active() {
                        TaskUpdate {
                            id: task_id.clone(),
                            update_status: TaskStatus::Active,
                            result_message: None,
                        }
                    } else {
                        continue;
                    };

                    let _ = client
                        .post("https://rci.dannyrs.xyz/task/update")
                        .header("Authorization", token.clone())
                        .json(&task)
                        .send()
                        .await?;
                }
            }

            for task_id in v_done_tasks_ids {
                println!("borrando: {}", task_id);
                guard.get_htasks_by_mut_ref().remove(&task_id);
            }
        }

        sleep(Duration::from_secs(10)).await;
    }
}
