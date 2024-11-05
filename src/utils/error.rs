use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum RciError {
    TestFailed,
    BuildFailed,
    CheckFailed,
    SysActionFailed(String),
    StdIO(std::io::Error),
    Unimplemented,
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
