use std::process::Command;

use serde::{Deserialize, Serialize};

use crate::utils::{error::RciError, shared_functions::run_command};

use super::{
    artifacts_output::ArtifactsOutput, instructions::Instruction,
    programming_languages::ProgrammingLanguage,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    name: String,
    language: ProgrammingLanguage,
    source: String,
    output: ArtifactsOutput,
    steps: Vec<Instruction>,
}

impl Task {
    pub fn execute(&self) -> Result<(), RciError> {
        // clone git repository
        run_command(&format!("git clone {}", self.source))?;

        // Execute every step
        for step in self.steps.iter() {
            step.do_instruction(&self.language)?;
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskList {
    pub tasks: Vec<Task>,
}

impl TaskList {
    pub fn run(&self) -> Result<(), RciError> {
        for task in self.tasks.iter() {
            task.execute()?;
        }
        Ok(())
    }
}
