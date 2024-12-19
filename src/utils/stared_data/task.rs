use serde::{Deserialize, Serialize};

use crate::utils::error::RciError;

use super::{Lang, Output, Step, TaskOutput};

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Task {
    pub lang: Lang,
    pub steps: Vec<Step>,
    pub source: String,
    pub output: Option<Output>,
}

impl Task {
    pub fn execute(&self) -> Result<TaskOutput, RciError> {
        for step in &self.steps {
            match step {
                Step::Check => {
                    println!("Compiling");
                }
                Step::Test => {
                    println!("Running");
                }
                Step::Build => {
                    println!("Building");
                }
                Step::Deploy => {
                    println!("Deploying");
                }
            }
        }

        Ok(TaskOutput::Pending)
    }
}
