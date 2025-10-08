use anyhow::Result;
use std::path::PathBuf;
use crate::stage::dependency_source_fetcher::fetch_dependency::FetchedDependency;

/**
    Converts a dependency tree into a flat dependency list.
 */
pub struct DependencyIncludeFetcher<'a> {
    dependencies: &'a [FetchedDependency]
}

impl<'a> DependencyIncludeFetcher<'a> {
    pub fn new(dependencies: &'a [FetchedDependency]) -> Self {
        Self {
            dependencies,
        }
    }

    /* Fetch dependencies header files */
    pub fn fetch(&self, include_dir: &PathBuf) -> Result<()> {
        for dependency in self.dependencies {
            
        }

        Ok(())
    }
}
