use std::{collections::HashMap, env, sync::Arc, time::Duration};

use rci_agent::{
    background::{
        auth::{self, auth},
        get_tasks::get_tasks,
    },
    instructions_management::tasks::TaskList,
    utils::{
        error::RciError,
        stared_data::{SharedData, TaskWrapper},
    },
};
use reqwest::Client;
use serde_json::from_str;
use tokio::{sync::Mutex, time::sleep};

#[tokio::main]
async fn main() -> Result<(), RciError> {
    let client = Client::new();
    let v_tasks: HashMap<String, TaskWrapper> = HashMap::new();
    let token = String::new();

    let shared_data = Arc::new(Mutex::new(SharedData::new(v_tasks, token, client)));

    let shared_data_clone = shared_data.clone();
    tokio::spawn(auth(shared_data_clone));
    while shared_data.lock().await.get_token_value_by_ref().is_empty() {
        sleep(Duration::from_millis(10)).await;
    }
    println!("token: {:?}", *shared_data.lock().await);

    let shared_data_clone = shared_data.clone();
    tokio::spawn(get_tasks(shared_data_clone));
    sleep(Duration::from_secs(10)).await;

    /*
    let args: Vec<String> = env::args().collect();

    let repo = args.get(1).ok_or(RciError::RequiredData)?;
    let json_str = r#"
        {
            "tasks": [
                {
                    "name": "task name",
                    "language": "Rust",
                    "output": "AzureBlobStorage",
                    "source": "",
                    "steps": [
                        {"Check": null},
                        {"Test": null},
                        {"Build": null},
                        {"Publish": null},
                    ]
                }
            ]
        }
    "#;

    let mut task_list: TaskList = from_str(json_str).unwrap();

    task_list.tasks[0].source = repo.clone();
    println!("{:#?}", &task_list);

    task_list.run().await?;*/
    Ok(())
}
