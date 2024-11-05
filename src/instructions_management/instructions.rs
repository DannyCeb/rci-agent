use serde::{Deserialize, Serialize};

use crate::utils::{error::RciError, shared_functions::run_command};

use super::programming_languages::ProgrammingLanguage;

#[derive(Serialize, Deserialize, Debug)]
pub enum Instruction {
    SysAction(String),
    Check,
    Test,
    Build,
    Publish,
}

impl Instruction {
    pub fn do_instruction(
        &self,
        lang: &ProgrammingLanguage,
        dir: Option<&str>,
    ) -> Result<(), RciError> {
        let dir = dir.unwrap_or(".");
        match self {
            Self::SysAction(command) => Ok(run_command(command, dir)?),
            Self::Check => match lang {
                ProgrammingLanguage::Rust => Ok(run_command("cargo check", dir)?),
                _ => {
                    eprintln!("Unsupported feature!");
                    Err(RciError::Unimplemented)
                }
            },
            Self::Test => match lang {
                ProgrammingLanguage::Rust => Ok(run_command("cargo test", dir)?),
                _ => {
                    eprintln!("Unsupported feature!");
                    Err(RciError::Unimplemented)
                }
            },
            Self::Build => match lang {
                ProgrammingLanguage::Rust => Ok(run_command("cargo build", dir)?),
                _ => {
                    eprintln!("Unsupported feature!");
                    Err(RciError::Unimplemented)
                }
            },
            _ => Ok(()),
        }
    }
}
