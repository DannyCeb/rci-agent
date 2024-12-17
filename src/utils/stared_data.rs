use std::collections::HashMap;

use reqwest::Client;
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Clone, Copy, Debug, Serialize)]
pub enum Lang {
    Rust,
    Csharp,
}

#[derive(Deserialize, Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum Step {
    Check,
    Test,
    Build,
    Deploy,
}

#[derive(Deserialize, Clone, Copy, Debug, Serialize)]
pub enum Output {
    StorageAccount,
    AzureContainerRegistry,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Task {
    pub lang: Lang,
    pub steps: Vec<Step>,
    pub source: String,
    pub output: Option<Output>,
}

#[derive(Serialize, Debug, Deserialize, Clone, Copy)]
pub enum TaskStatus {
    Pending,
    Active,
    Done,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub enum TaskOutput {
    Pending,
    Result(String),
}

#[derive(Serialize, Debug, Deserialize)]
pub struct TaskWrapper {
    pub id: String,
    pub task: Task,
    pub status: TaskStatus,
    pub output: TaskOutput,
}

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

    pub fn get_http_client_by_mut_ref(&mut self) -> &mut Client {
        &mut self.client
    }

    pub fn get_htasks_by_mut_ref(&mut self) -> &mut HashMap<String, TaskWrapper> {
        &mut self.h_tasks
    }
}
