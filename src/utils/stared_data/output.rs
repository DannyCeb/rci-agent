use std::fs;

use azure_storage::StorageCredentials;
use azure_storage_blobs::prelude::ClientBuilder;
use env_file_reader::read_file;
use serde::{Deserialize, Serialize};

use crate::utils::error::RciError;
#[derive(Deserialize, Clone, Copy, Debug, Serialize)]
pub enum Output {
    StorageAccount,
    AzureContainerRegistry,
}

impl Output {
    pub async fn deploy(&self, workdir: &str) -> Result<String, RciError> {
        match self {
            Output::StorageAccount => {
                println!("Deploying Storage Account");
                let blob_name = workdir.split("/").collect::<Vec<&str>>();

                let blob_name = blob_name.last().unwrap();
                let blob_path = &format!("{workdir}/target/debug/{blob_name}");

                Ok(self.publish_to_azure(blob_name, blob_path).await?)
            }
            Output::AzureContainerRegistry => {
                println!("Deploying Azure Container Registry");
                Ok("unimplemented output".into())
            }
        }
    }

    pub async fn publish_to_azure(
        &self,
        blob_name: &str,
        blob_path: &str,
    ) -> Result<String, RciError> {
        let env_variables = read_file(".env")?;
        let account = env_variables
            .get("STORAGE_ACCOUNT_NAME")
            .ok_or(RciError::EnvFileError)?;
        let access_key = env_variables
            .get("STORAGE_ACCOUNT_KEY")
            .ok_or(RciError::EnvFileError)?;
        let container = env_variables
            .get("STORAGE_CONTAINER_NAME")
            .ok_or(RciError::EnvFileError)?;

        let storage_credentials =
            StorageCredentials::access_key(account.clone(), access_key.clone());
        let blob_client =
            ClientBuilder::new(account, storage_credentials).blob_client(container, blob_name);

        let contents = fs::read(blob_path)?;

        blob_client
            .put_block_blob(contents)
            .content_type("binary")
            .await?;

        Ok(format!(
            "https://rciartifacts.blob.core.windows.net/devartifacts/{blob_name}"
        ))
    }
}
