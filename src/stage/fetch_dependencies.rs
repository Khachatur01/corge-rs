use std::path::PathBuf;
use crate::stage::resolve_dependencies::DependencySource;

pub struct DependencyFetcher {
    dependency_sources: Vec<DependencySource>,
}

impl DependencyFetcher {
    pub fn new(dependency_sources: Vec<DependencySource>,) -> Self {
        Self {
            dependency_sources,
        }
    }

    pub fn fetch(&self, dependency_directory: &PathBuf) {
        
    }
}
