pub mod fetch_dependency;

use crate::config::{Config, Dependency, Registry};
use crate::stage::dependency_source_fetcher::fetch_dependency::{fetch_dependency, FetchedDependency};
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

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
    pub fn fetch(&self, sources_dir: &PathBuf) -> Result<Vec<FetchedDependency>> {
        let mut fetched_dependencies = vec![];

        for dependency in &self.dependencies {
            /* todo: check if dependency is already fetched */

            let registry = self.registries.get(&dependency.registry_name)
                .ok_or_else(|| anyhow::anyhow!("Repository '{}' not found", &dependency.registry_name))?;

            fetch_dependency(registry, dependency, sources_dir)?;

            fetched_dependencies.push(
                FetchedDependency {
                    source: sources_dir.join(&dependency.name),
                    dependency: dependency.clone()
                }
            );

            let config_path = sources_dir.join(&dependency.name).join("build.yaml");
            let config_str = fs::read_to_string(config_path)
                .with_context(|| format!("Failed to read build.yaml for dependency {}", dependency.name))?;

            let config: Config = serde_yaml::from_str(&config_str)
                .with_context(|| format!("Failed to parse build.yaml for dependency {}", dependency.name))?;

            let fetched_child_dependencies = DependencySourceFetcher::new(config.registries, config.dependencies)
                .fetch(sources_dir)
                .with_context(|| format!("Failed to fetch dependencies for dependency {}", dependency.name))?;

            fetched_dependencies.extend(fetched_child_dependencies);
        }

        Ok(fetched_dependencies)
    }
}
