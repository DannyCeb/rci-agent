use serde::{Deserialize, Serialize};

use crate::utils::{error::RciError, shared_functions::run_command};
#[derive(Deserialize, Clone, Copy, Debug, Serialize)]
pub enum Lang {
    Rust,
    Csharp,
}

/// Represents a programming language and provides methods to compile, test, and check code.
///
/// # Methods
///
/// * `compile(&self, workdir: &str) -> Result<(), RciError>`
///     - Compiles the code in the specified working directory.
///     - For `Lang::Rust`, it runs `cargo build`.
///     - For `Lang::Csharp`, it prints a message (implementation needed).
///
/// * `test(&self, workdir: &str) -> Result<(), RciError>`
///     - Tests the code in the specified working directory.
///     - For `Lang::Rust`, it runs `cargo test`.
///     - For `Lang::Csharp`, it prints a message (implementation needed).
///
/// * `check(&self, workdir: &str) -> Result<(), RciError>`
///     - Checks the code in the specified working directory.
///     - For `Lang::Rust`, it runs `cargo check`.
///     - For `Lang::Csharp`, it prints a message (implementation needed).
///
/// # Parameters
///
/// * `workdir: &str` - The working directory where the commands should be executed.
///
/// # Errors
///
/// Returns an `RciError` if the command execution fails.
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
