use std::env;

use rci_agent::{instructions_management::tasks::TaskList, utils::error::RciError};
use serde_json::from_str;

#[tokio::main]
async fn main() -> Result<(), RciError> {
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

    task_list.run().await?;
    Ok(())
}
