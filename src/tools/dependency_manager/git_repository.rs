use crate::config::Dependency;
use std::path::Path;
use std::process::Command;
use anyhow::{Context, Result};
use crate::std_command_ext::{ExecuteCommand};

pub fn fetch_git_dependency(url: &str, branch: &str, dependency: &Dependency, source_directory: &Path) -> Result<()> {
    let mut command = Command::new("git");
    command.arg("clone");
    command.arg(format!("{}/{}", url, dependency.name));
    command.arg("--single-branch");
    command.arg("--branch");
    command.arg(branch);
    command.arg(source_directory.join("source").join(&dependency.name));

    command.execute(true)?;

    Ok(())
}
