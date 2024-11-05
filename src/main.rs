use rci_agent::{instructions_management::tasks::TaskList, utils::error::RciError};
use serde_json::from_str;

fn main() -> Result<(), RciError> {
    let json_str = r#"
        {
            "tasks": [
                {
                    "name": "task name",
                    "language": "Rust",
                    "output": "ContainerRegistry",
                    "source": "github repository",
                    "steps": [
                        {"Check": null},
                        {"Test": null},
                        {"Build": null},
                        {"Publish": null},
                        {"SysAction": "ls"}
                    ]
                }
            ]
        }
    "#;

    let task_list: TaskList = from_str(json_str).unwrap();

    println!("{:#?}", &task_list);

    task_list.run()?;
    Ok(())
}
