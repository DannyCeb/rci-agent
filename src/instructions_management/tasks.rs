use crate::utils::{error::RciError, shared_functions::run_command};
use azure_storage::prelude::*;
use azure_storage_blobs::prelude::*;
use env_file_reader::read_file;
use serde::{Deserialize, Serialize};
use std::fs;

use super::{
    artifacts_output::ArtifactsOutput, instructions::Instruction,
    programming_languages::ProgrammingLanguage,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    name: String,
    language: ProgrammingLanguage,
    pub source: String,
    output: ArtifactsOutput,
    steps: Vec<Instruction>,
}

impl Task {
    pub async fn execute(&self) -> Result<(), RciError> {
        println!("[Info]: Executing {}", self.name);
        // clone git repository
        run_command(&format!("git clone {}", &self.source), ".")?;
        let dir = self.source.split("/").collect::<Vec<&str>>();

        let dir = *dir.last().unwrap();
        println!("[Info]: Git dir: {}", dir);

        // Execute every step
        for step in self.steps.iter() {
            println!("[Info]: doing: {:?}", step);
            step.do_instruction(&self.language, Some(dir))?;
        }

        self.publish().await?;

        self.cleanup()?;
        // celan dir
        Ok(())
    }

    pub async fn publish(&self) -> Result<(), RciError> {
        match self.output {
            ArtifactsOutput::ContainerRegistry => {
                // Create the build image and the push logic
                Ok(())
            }
            ArtifactsOutput::AzureBlobStorage => {
                // Create the upload logic
                println!("[Info]: Publishing to Azure Blob Storage");
                let blob_name = self.source.split("/").collect::<Vec<&str>>();

                let blob_name = *blob_name.last().unwrap();
                let blob_path = &format!("{blob_name}/target/debug/{blob_name}");
                self.publish_to_azure(blob_name, blob_path).await?;
                Ok(())
            }
        }
    }

    pub async fn publish_to_azure(&self, blob_name: &str, blob_path: &str) -> Result<(), RciError> {
        let env_variables = read_file(".env")?;
        let account = env_variables.get("storage_account").unwrap();
        let access_key = env_variables.get("access_key").unwrap();
        let container = env_variables.get("container").unwrap();

        let storage_credentials =
            StorageCredentials::access_key(account.clone(), access_key.clone());
        let blob_client =
            ClientBuilder::new(account, storage_credentials).blob_client(container, blob_name);

        let contents = fs::read(blob_path)?;

        blob_client
            .put_block_blob(contents)
            .content_type("binary")
            .await?;

        println!("Blob uri: https://rciartifacts.blob.core.windows.net/devartifacts/{blob_name}");
        Ok(())
    }

    pub fn cleanup(&self) -> Result<(), RciError> {
        let dir = self.source.split("/").collect::<Vec<&str>>();

        let dir = *dir.last().unwrap();
        println!("[Info]: removed dir: {}", dir);

        run_command(&format!("rm -rf {}", dir), ".")?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskList {
    pub tasks: Vec<Task>,
}

impl TaskList {
    pub async fn run(&self) -> Result<(), RciError> {
        for task in self.tasks.iter() {
            task.execute().await?;
        }
        Ok(())
    }
}
