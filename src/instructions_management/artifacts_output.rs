use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]

pub enum ArtifactsOutput {
    ContainerRegistry,
    AzureBlobStorage,
}
