use serde::{Deserialize, Serialize};

use super::{Task, TaskOutput, TaskStatus};

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct TaskWrapper {
    pub id: String,
    pub task: Task,
    pub status: TaskStatus,
    pub output: TaskOutput,
}

impl TaskWrapper {
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
}
