mod lang;
mod output;
mod step;
mod task;
mod task_output;
mod task_status;
mod task_wrapper;

pub use lang::Lang;
pub use output::Output;
pub use step::Step;
pub use task::Task;
pub use task_output::TaskOutput;
pub use task_status::TaskStatus;
pub use task_wrapper::TaskWrapper;

use reqwest::Client;
use std::collections::HashMap;

#[derive(Debug)]
pub struct SharedData {
    h_tasks: HashMap<String, TaskWrapper>,
    token: String,
    client: Client,
}

impl SharedData {
    pub fn new(h_tasks: HashMap<String, TaskWrapper>, token: String, client: Client) -> Self {
        Self {
            h_tasks,
            token,
            client,
        }
    }

    pub fn set_token(&mut self, token: String) {
        self.token = token
    }

    pub fn get_token_value_by_ref(&self) -> &String {
        &self.token
    }

    pub fn get_http_client_by_ref(&self) -> &Client {
        &self.client
    }

    pub fn get_htasks_by_mut_ref(&mut self) -> &mut HashMap<String, TaskWrapper> {
        &mut self.h_tasks
    }

    pub fn get_htasks_by_ref(&self) -> &HashMap<String, TaskWrapper> {
        &self.h_tasks
    }

    pub fn get_task_clone_by_id(&self, id: &String) -> Option<TaskWrapper> {
        match self.h_tasks.get(id) {
            Some(task) => Some(task.clone()),
            None => None,
        }
    }
}
