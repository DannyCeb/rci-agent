use serde::{Deserialize, Serialize};

use crate::utils::error::RciError;

use super::{Lang, Output, Step, TaskOutput};

#[derive(Deserialize, Debug, Serialize, Clone)]
/// Represents a task with a specific language, a series of steps, a source, and an optional output.
///
/// # Fields
///
/// * `lang` - The programming language associated with the task.
/// * `steps` - A vector of steps to be executed as part of the task.
/// * `source` - The source code or script for the task.
/// * `output` - An optional output that may be produced by the task.
pub struct Task {
    pub lang: Lang,
    pub steps: Vec<Step>,
    pub source: String,
    pub output: Option<Output>,
}

/// Executes the task by iterating through its steps and performing the corresponding actions.
///
/// # Arguments
///
/// * `workdir` - A string slice that holds the working directory path.
///
/// # Returns
///
/// * `Result<TaskOutput, RciError>` - Returns a `TaskOutput` on success or an `RciError` on failure.
///
/// # Steps
///
/// * `Step::Check` - Performs a check using the language-specific implementation.
/// * `Step::Test` - Runs tests using the language-specific implementation.
/// * `Step::Build` - Compiles the code using the language-specific implementation.
/// * `Step::Deploy` - Deploys the output and returns the artifact URL.
///
/// If the `Step::Deploy` step is reached and executed, the function returns early with the deployment result.
/// Otherwise, it returns a success message after all steps are executed.
impl Task {
    pub async fn execute(&self, workdir: &str) -> Result<TaskOutput, RciError> {
        for step in &self.steps {
            match step {
                Step::Check => {
                    self.lang.check(workdir)?;
                }
                Step::Test => {
                    self.lang.test(workdir)?;
                }
                Step::Build => {
                    self.lang.compile(workdir)?;
                }
                Step::Deploy => {
                    let artifact_url = self.output.unwrap().deploy(workdir).await?;
                    return Ok(TaskOutput::Result(format!(
                        "Artifact url: {}",
                        artifact_url
                    )));
                }
            }
        }

        Ok(TaskOutput::Result("Task executed successfully".into()))
    }
}
