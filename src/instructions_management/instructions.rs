use crate::utils::error::RciError;
use std::process::Command;

pub enum InstructionKind {
    SysAction(String),
    Check(ProgrammingLanguage),
    Test(ProgrammingLanguage),
    Build(ProgrammingLanguage),
    Publish(ArtifactsOutput),
}

impl InstructionKind {
    pub fn do_instruction(&self) -> Result<(), RciError> {
        match self {
            Self::SysAction(command) => Ok(run_command(command)?),
            Self::Check(lang) => match lang {
                ProgrammingLanguage::Rust => Ok(run_command("cargo check")?),
                _ => {
                    eprintln!("Unsupported feature!");
                    Err(RciError::Unimplemented)
                }
            },
            Self::Test(lang) => match lang {
                ProgrammingLanguage::Rust => Ok(run_command("cargo test")?),
                _ => {
                    eprintln!("Unsupported feature!");
                    Err(RciError::Unimplemented)
                }
            },
            Self::Build(lang) => match lang {
                ProgrammingLanguage::Rust => Ok(run_command("cargo build")?),
                _ => {
                    eprintln!("Unsupported feature!");
                    Err(RciError::Unimplemented)
                }
            },
            _ => Err(RciError::Unimplemented),
        }
    }
}

pub enum ProgrammingLanguage {
    Rust,
    Python,
}

pub enum ArtifactsOutput {
    ContainerRegistry,
    AzureBlobStorage,
}

fn run_command(command: &str) -> Result<(), RciError> {
    let output = Command::new("bash").arg("-c").arg(command).output()?;

    if output.status.success() {
        Ok(())
    } else {
        Err(RciError::SysActionFailed)
    }
}
