use std::process::Command;

use super::error::RciError;

pub fn run_command(command: &str, dir: &str) -> Result<(), RciError> {
    let output = Command::new("bash")
        .current_dir(dir)
        .arg("-c")
        .arg(command)
        .output()?;

    if output.status.success() {
        Ok(())
    } else {
        Err(RciError::SysActionFailed(
            String::from_utf8_lossy(&output.stderr).into_owned(),
        ))
    }
}
