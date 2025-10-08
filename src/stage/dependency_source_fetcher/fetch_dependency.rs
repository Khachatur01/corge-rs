use std::path::PathBuf;
use anyhow::Context;
use crate::config::{Dependency, Registry};

mod fs_repository;
mod git_repository;

pub struct FetchedDependency {
    pub source: PathBuf,
    pub dependency: Dependency,
}

pub fn fetch_dependency(registry: &Registry, dependency: &Dependency, sources_directory: &PathBuf) -> anyhow::Result<()> {
    match registry {
        Registry::Git { url, branch } => {
            log::info!("Fetching dependency '{}' from 'git' repository {}", dependency.name, url);
            git_repository::fetch_git_dependency(
                url,
                branch,
                dependency,
                sources_directory
            ).with_context(|| format!("Failed to fetch dependency '{}' from 'git' repository {}", dependency.name, url))
        },
        Registry::FileSystem(repository_path) => {
            log::info!("Fetching dependency '{}' from 'fs' repository {:?}", dependency.name, repository_path);
            fs_repository::fetch_fs_dependency(
                repository_path.as_ref(),
                dependency,
                sources_directory
            ).with_context(|| format!("Failed to fetch dependency '{}' from 'fs' repository {}", dependency.name, repository_path))
        }
    }
}
