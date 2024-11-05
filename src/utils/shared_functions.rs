use std::process::Command;

use super::error::RciError;

pub fn run_command(command: &str) -> Result<(), RciError> {
    let output = Command::new("bash").arg("-c").arg(command).output()?;

    if output.status.success() {
        Ok(())
    } else {
        Err(RciError::SysActionFailed(
            String::from_utf8_lossy(&output.stderr).to_lowercase(),
        ))
    }
}
