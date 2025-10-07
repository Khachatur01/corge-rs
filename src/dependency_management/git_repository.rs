use std::path::Path;
use crate::model::{Config, Dependency};

pub fn copy_git_dependency(url: &str, branch: &str, dependency: &Dependency, output_directory: &Path) -> Config {
    println!("Copying dependency '{}' from 'git' repository {}", dependency.name, url);

    /* todo: check if dependency already exists */
    Config::default()
}
