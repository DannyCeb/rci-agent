use std::{
    sync::{Arc, MutexGuard},
    time::Duration,
};

use serde::{Deserialize, Serialize};
use tokio::{sync::Mutex, time::sleep};

use crate::utils::{
    error::RciError,
    stared_data::{SharedData, TaskWrapper},
};

#[derive(Serialize, Debug, Deserialize)]
struct TasksRespose {
    tasks: Vec<TaskWrapper>,
}

pub async fn get_tasks(shared_data: Arc<Mutex<SharedData>>) -> Result<(), RciError> {
    loop {
        let tasks_received = {
            let mut guard = shared_data.lock().await;
            let token = guard.get_token_value_by_ref().clone();
            let client = guard.get_http_client_by_mut_ref();
            let response = client
                .get("https://rci.dannyrs.xyz/task/list_tasks")
                .header("Authorization", token)
                .send()
                .await?;

            response.json::<TasksRespose>().await.unwrap()
        };

        insert_tasks(shared_data.clone(), tasks_received).await;
        sleep(Duration::from_secs(10)).await
    }
}

async fn insert_tasks(shared_data: Arc<Mutex<SharedData>>, tasks_received: TasksRespose) {
    let tasks = tasks_received.tasks;

    let mut guard = shared_data.lock().await;

    let shared_tasks = guard.get_htasks_by_mut_ref();

    for task in tasks {
        let id = task.id.clone();

        shared_tasks.entry(id).or_insert(task);
    }
}
