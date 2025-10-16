mod fs_registry;
mod git_registry;

use crate::config::{Config, Dependency, Registry};
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;


pub struct Artifact {
    pub path: PathBuf,
    pub dependency: Dependency,
}

/**
    Converts a dependency tree into a flat dependency list.
 */
pub struct DependencySourceFetcher {
    registries: HashMap<String, Registry>,
    dependencies: Vec<Dependency>,
}

impl DependencySourceFetcher {
    pub fn new(registries: HashMap<String, Registry>, dependencies: Vec<Dependency>,) -> Self {
        Self {
            registries,
            dependencies,
        }
    }

    /* Fetch dependencies recursively and return a flat vector */
    pub fn fetch(&self, sources_dir: &PathBuf) -> Result<Vec<Artifact>> {
        let mut artifacts = vec![];

        for dependency in &self.dependencies {
            let artifact_path = sources_dir.join(&dependency.name);

            let registry = self.registries.get(&dependency.registry_name)
                .ok_or_else(|| anyhow::anyhow!("Registry '{}' not found", &dependency.registry_name))?;

            let dependency_fetched = fs::exists(&artifact_path)
                .with_context(|| format!("Failed to check if dependency '{}' is fetched", dependency.name))?;

            if dependency_fetched {
                log::info!("Skipping already fetched dependency '{}'", dependency.name);
            } else {
                fetch_dependency(registry, dependency, &artifact_path)?;
            }

            artifacts.push(
                Artifact {
                    path: sources_dir.join(&dependency.name),
                    dependency: dependency.clone()
                }
            );

            let config_path = sources_dir.join(&dependency.name).join("build.yaml");
            let config_str = fs::read_to_string(config_path)
                .with_context(|| format!("Failed to read build.yaml for dependency {}", dependency.name))?;

            let config: Config = serde_yaml::from_str(&config_str)
                .with_context(|| format!("Failed to parse build.yaml for dependency {}", dependency.name))?;

            let children_artifacts = DependencySourceFetcher::new(config.registries, config.dependencies)
                .fetch(sources_dir)
                .with_context(|| format!("Failed to fetch dependencies for dependency {}", dependency.name))?;

            artifacts.extend(children_artifacts);
        }

        Ok(artifacts)
    }
}

fn fetch_dependency(registry: &Registry, dependency: &Dependency, artifact_path: &PathBuf) -> Result<()> {
    match registry {
        Registry::Git { url, branch } => {
            log::info!("Fetching dependency '{}' from 'git' repository {}", dependency.name, url);
            git_registry::fetch_git_dependency(
                url,
                branch,
                dependency,
                artifact_path
            ).with_context(|| format!("Failed to fetch dependency '{}' from 'git' repository {}", dependency.name, url))
        },
        Registry::FileSystem(repository_path) => {
            log::info!("Fetching dependency '{}' from 'fs' repository {:?}", dependency.name, repository_path);
            fs_registry::fetch_fs_dependency(
                repository_path.as_ref(),
                dependency,
                artifact_path
            ).with_context(|| format!("Failed to fetch dependency '{}' from 'fs' repository {}", dependency.name, repository_path))
        }
    }
}
