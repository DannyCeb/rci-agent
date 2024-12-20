/// `RciError` is an enumeration of possible errors that can occur in the RCI agent.
///
/// # Variants
///
/// - `TestFailed`: Indicates that a test has failed.
/// - `BuildFailed`: Indicates that a build has failed.
/// - `CheckFailed`: Indicates that a check has failed.
/// - `SysActionFailed(String)`: Indicates that a system action has failed, with a message describing the failure.
/// - `StdIO(std::io::Error)`: Represents an I/O error.
/// - `Unimplemented`: Indicates that a feature is unimplemented.
/// - `AzureStorageError(azure_storage::Error)`: Represents an error from Azure Storage.
/// - `RequiredData`: Indicates that required data is missing.
/// - `EnvFileError`: Indicates an error related to environment files.
/// - `ReqwestError(reqwest::Error)`: Represents an error from the Reqwest HTTP client.
/// - `MissingInformation(String)`: Indicates that some required information is missing, with a message describing the missing information.
use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum RciError {
    TestFailed,
    BuildFailed,
    CheckFailed,
    SysActionFailed(String),
    StdIO(std::io::Error),
    Unimplemented,
    AzureStorageError(azure_storage::Error),
    RequiredData,
    EnvFileError,
    ReqwestError(reqwest::Error),
    MissingInformation(String),
}

impl Display for RciError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for RciError {}

impl From<std::io::Error> for RciError {
    fn from(error: std::io::Error) -> Self {
        Self::StdIO(error)
    }
}

impl From<azure_storage::Error> for RciError {
    fn from(error: azure_storage::Error) -> Self {
        Self::AzureStorageError(error)
    }
}

impl From<reqwest::Error> for RciError {
    fn from(error: reqwest::Error) -> Self {
        Self::ReqwestError(error)
    }
}
