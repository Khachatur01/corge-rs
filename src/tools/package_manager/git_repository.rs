use crate::config::Dependency;
use std::path::Path;
use std::process::Command;

pub fn copy_git_dependency(url: &str, branch: &str, dependency: &Dependency, source_directory: &Path) {
    println!("Copying dependency '{}' from 'git' repository {}", dependency.name, url);

    let mut command = Command::new("git");
    command.arg("clone");
    command.arg(format!("{}/{}", url, dependency.name));
    command.arg("--single-branch");
    command.arg("--branch");
    command.arg(branch);
    command.arg(source_directory.join("source").join(&dependency.name));

    println!("{:?}", command);

    command.output().unwrap();
}
