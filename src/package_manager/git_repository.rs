use std::fs;
use crate::config::{Config, Dependency};
use std::path::Path;
use std::process::Command;
use crate::package_manager::copy_headers;

pub fn copy_git_dependency(url: &str, branch: &str, dependency: &Dependency, output_directory: &Path) -> Option<Config> {
    println!("Copying dependency '{}' from 'git' repository {}", dependency.name, url);

    /* todo: check if dependency already exists */

    let header_dir = &output_directory.join("header").join(&dependency.name);
    let source_dir = &output_directory.join("source").join(&dependency.name);

    let mut command = Command::new("git");
    command.arg("clone");
    command.arg(format!("{}/{}", url, dependency.name));
    command.arg("--single-branch");
    command.arg("--branch");
    command.arg(branch);
    command.arg(output_directory.join("source").join(&dependency.name));

    println!("{:?}", command);

    command.output().unwrap();
    copy_headers(&source_dir.join("src"), header_dir);

    fs::read_to_string(source_dir.join("build.yaml"))
        .ok()
        .map(|config_str: String| serde_yaml::from_str(&config_str))?
        .ok()
}
