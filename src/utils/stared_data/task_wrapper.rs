use serde::{Deserialize, Serialize};

use crate::utils::{error::RciError, shared_functions::run_command};

use super::{Task, TaskOutput, TaskStatus};

#[derive(Serialize, Debug, Deserialize, Clone)]
/// A struct that wraps a task with additional metadata.
///
/// `TaskWrapper` contains the following fields:
/// - `id`: A unique identifier for the task.
/// - `task`: The actual task to be executed.
/// - `status`: The current status of the task.
/// - `output`: The output produced by the task.
pub struct TaskWrapper {
    id: String,
    task: Task,
    status: TaskStatus,
    output: TaskOutput,
}

/// A wrapper for tasks that provides various utility methods.
///
/// # Methods
///
/// - `get_id_by_ref(&self) -> &String`
///   Returns a reference to the task's ID.
///
/// - `is_done(&self) -> bool`
///   Checks if the task status is `Done`.
///
/// - `is_active(&self) -> bool`
///   Checks if the task status is `Active`.
///
/// - `get_result(&self) -> Option<String>`
///   Retrieves the result of the task if available.
///
/// - `set_status(&mut self, status: TaskStatus)`
///   Sets the status of the task.
///
/// - `set_output(&mut self, output: TaskOutput)`
///   Sets the output of the task.
///
/// - `execute(&mut self) -> Result<(), RciError>`
///   Asynchronously executes the task, setting its output and status upon completion.
impl TaskWrapper {
    pub fn get_id_by_ref(&self) -> &String {
        &self.id
    }

    pub fn is_done(&self) -> bool {
        self.status == TaskStatus::Done
    }

    pub fn is_active(&self) -> bool {
        self.status == TaskStatus::Active
    }

    pub fn get_result(&self) -> Option<String> {
        match &self.output {
            TaskOutput::Pending => None,
            TaskOutput::Result(s) => Some(s.clone()),
        }
    }

    pub fn set_status(&mut self, status: TaskStatus) {
        self.status = status;
    }

    pub fn set_output(&mut self, output: TaskOutput) {
        self.output = output;
    }

    pub async fn execute(&mut self) -> Result<(), RciError> {
        let workdir = format!("/tmp/{}", self.id);
        run_command(&format!("mkdir {}", workdir), "/")?;

        run_command(&format!("git clone {}", &self.task.source), &workdir)?;

        let dir_name = self.task.source.split("/").collect::<Vec<&str>>();

        let dir_name = *dir_name.last().unwrap();

        let workdir = format!("{}/{}", workdir, dir_name);

        let result = self.task.execute(&workdir).await?;
        self.set_output(result);
        self.set_status(TaskStatus::Done);
        Ok(())
    }
}
