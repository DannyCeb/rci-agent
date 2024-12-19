use serde::{Deserialize, Serialize};

use super::{Task, TaskOutput, TaskStatus};

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct TaskWrapper {
    id: String,
    task: Task,
    status: TaskStatus,
    output: TaskOutput,
}

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
}
