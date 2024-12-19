use serde::{Deserialize, Serialize};

use crate::utils::{error::RciError, shared_functions::run_command};
#[derive(Deserialize, Clone, Copy, Debug, Serialize)]
pub enum Lang {
    Rust,
    Csharp,
}

impl Lang {
    pub fn compile(&self, workdir: &str) -> Result<(), RciError> {
        match self {
            Lang::Rust => {
                println!("Compiling Rust");
                run_command("cargo build", workdir)?;
            }
            Lang::Csharp => {
                println!("Compiling C#");
            }
        }
        Ok(())
    }

    pub fn test(&self, workdir: &str) -> Result<(), RciError> {
        match self {
            Lang::Rust => {
                println!("Testing Rust");
                run_command("cargo test", workdir)?;
            }
            Lang::Csharp => {
                println!("Testing C#");
            }
        }
        Ok(())
    }

    pub fn check(&self, workdir: &str) -> Result<(), RciError> {
        match self {
            Lang::Rust => {
                println!("Checking Rust");
                run_command("cargo check", workdir)?;
            }
            Lang::Csharp => {
                println!("Checking C#");
            }
        }
        Ok(())
    }
}
