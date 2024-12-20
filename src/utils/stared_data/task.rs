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
