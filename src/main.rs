use std::{env, sync::Arc, time::Duration};

use rci_agent::{
    background::auth::{self, auth},
    instructions_management::tasks::TaskList,
    utils::error::RciError,
};
use serde_json::from_str;
use tokio::{sync::Mutex, time::sleep};

#[tokio::main]
async fn main() -> Result<(), RciError> {
    let token = Arc::new(Mutex::new(String::new()));
    let token_clone = token.clone();
    tokio::spawn(async move { auth(token_clone).await });

    println!("Antes de dormir");
    sleep(Duration::from_secs(2)).await;

    println!("token: {}", *token.clone().lock().await);

    sleep(Duration::from_secs(7)).await;

    println!("token: {}", *token.clone().lock().await);

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
