use crate::config::{Dependency, LinkStrategy};
use crate::tools::source_manager::find_source_files;
use std::path::PathBuf;

#[derive(Clone)]
pub struct Manifest {
    pub name: String,
    pub link_strategy: LinkStrategy,
    pub sources: Vec<PathBuf>,
}

fn fetch_dependency_sources(project_dir: &PathBuf, dependency_name: &str) -> Vec<PathBuf> {
    let sources_dir = project_dir
        .join("dependency")
        .join("source")
        .join(dependency_name)
        .join("src");

    find_source_files(&sources_dir)
}

pub fn get_dependency_manifests(project_dir: &PathBuf, dependencies: &[Dependency]) -> Vec<Manifest> {
    dependencies
        .iter()
        .map(|dependency|
            Manifest {
                name: dependency.name.clone(),
                link_strategy: dependency.link_strategy.clone(),
                sources: fetch_dependency_sources(project_dir, &dependency.name),
            }
        )
        .collect()
}
