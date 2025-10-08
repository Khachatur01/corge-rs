mod fs_repository;
mod git_repository;

use crate::config::{Config, Dependency, Registry};
use crate::tools::dependency_manager::fs_repository::fetch_fs_dependency;
use crate::tools::dependency_manager::git_repository::fetch_git_dependency;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use anyhow::{Context, Result};

fn copy_headers(src_path: &PathBuf, dst_path: &PathBuf) -> Result<()> {
    let source_dir = fs::read_dir(src_path)?;

    for source_file in source_dir {
        let source_file = source_file?;

        if source_file.path().is_dir() {
            copy_headers(
                &source_file.path(),
                &dst_path.join(source_file.file_name())
            )?;
        } else {
            fs::create_dir_all(dst_path)?;

            let is_header_file = source_file
                .path()
                .extension()
                .map(|extension| extension == "h")
                .unwrap_or(false);

            if is_header_file {
                fs::copy(
                    source_file.path(),
                    dst_path.join(source_file.file_name())
                )?;
            }
        }
    }

    Ok(())
}

fn fetch_dependency(registry: &Registry, dependency: &Dependency, output_directory: &Path) -> Result<()> {
    match registry {
        Registry::Git { url, branch } => {
            log::info!("Fetching dependency '{}' from 'git' repository {}", dependency.name, url);
            fetch_git_dependency(
                url,
                branch,
                dependency,
                output_directory
            ).with_context(|| format!("Failed to fetch dependency '{}' from 'git' repository {}", dependency.name, url))
        },
        Registry::FileSystem(repository_path) => {
            log::info!("Fetching dependency '{}' from 'fs' repository {:?}", dependency.name, repository_path);
            fetch_fs_dependency(
                repository_path.as_ref(),
                dependency,
                output_directory
            ).with_context(|| format!("Failed to fetch dependency '{}' from 'fs' repository {}", dependency.name, repository_path))
        }
    }
}

pub fn fetch_dependencies(
    repositories: &HashMap<String, Registry>,
    dependencies: &[Dependency],
    output_directory: &Path
) -> Result<()> {
    for dependency in dependencies {
        log::info!(dependency:? = dependency.name; "Resolving");

        let repository_name = &dependency.repository_name;

        let repository = repositories.get(repository_name)
            .ok_or_else(|| anyhow::anyhow!("Repository '{}' not found", repository_name))?;

        let header_dir = &output_directory.join("header").join(&dependency.name);
        let source_dir = &output_directory.join("source").join(&dependency.name);

        log::info!(dependency:? = dependency.name; "Header directory {}", header_dir.display());
        log::info!(dependency:? = dependency.name; "Source directory {}", source_dir.display());

        let source_dir_exists = fs::exists(source_dir)
            .with_context(|| format!("Failed to check if source directory exists for dependency '{}'", dependency.name))?;

        if source_dir_exists {
            log::info!(dependency:? = dependency.name; "Directory exists. Skip fetching...");
            continue;
        }

        fetch_dependency(repository, &dependency, source_dir)?;

        copy_headers(&source_dir.join("src"), header_dir)
            .with_context(|| format!("Failed to copy headers for dependency {}", dependency.name))?;

        let config: Option<Config> = fs::read_to_string(source_dir.join("build.yaml"))
            .ok()
            .map(|config_str: String| serde_yaml::from_str(&config_str).ok())
            .flatten();

        let Some(config) = config else {
            log::info!("No build.yaml found for dependency {}. Skip resolving dependencies...", dependency.name);
            continue;
        };

        fetch_dependencies(
            &config.repositories,
            &dependencies,
            output_directory
        ).with_context(|| format!("Failed to resolve dependencies for dependency {}", dependency.name))?;
    }

    Ok(())
}
