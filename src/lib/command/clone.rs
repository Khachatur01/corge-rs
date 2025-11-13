use std::path::PathBuf;
use std::process::Command;
use crate::cli::{CloneArgs, CloneSource};
use crate::std_command_ext::ExecuteCommand;
use crate::tool::dir_copier::deep_copy;

fn clone_git(url: String, branch: String, destination: PathBuf) -> anyhow::Result<()> {
    if destination.exists() {
        std::fs::remove_dir_all(&destination)?;
    }

    let mut command = Command::new("git");
    command.arg("clone");
    command.arg(url);
    command.arg("--single-branch");
    command.arg("--branch");
    command.arg(branch);
    command.arg(destination);

    command.execute(true)?;

    Ok(())
}

fn clone_file_system(from_path: PathBuf, destination: PathBuf) -> anyhow::Result<()> {
    deep_copy(from_path, destination)?;

    Ok(())
}

pub fn clone(clone_args: CloneArgs) -> anyhow::Result<()> {
    log::info!("Cloning project from {:?} to {:?}", clone_args.source, clone_args.path);

    match clone_args.source {
        CloneSource::Git { url, branch } => clone_git(url, branch, clone_args.path)?,
        CloneSource::FileSystem { from } => clone_file_system(from, clone_args.path)?,
    }

    log::info!("PROJECT CLONED SUCCESSFULLY");
    Ok(())
}
