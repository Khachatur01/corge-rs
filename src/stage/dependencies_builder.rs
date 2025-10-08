use anyhow::Result;
use std::path::PathBuf;
use crate::command::build::target_path::TargetDependencyPath;
use crate::config::{Profile, Toolchain};
use crate::stage::dependency_source_fetcher::fetch_dependency::FetchedDependency;

pub struct DependenciesBuilder<'a> {
    toolchain: Toolchain,
    profile: Profile,
    dependencies: &'a [FetchedDependency]
}

impl<'a> DependenciesBuilder<'a> {
    pub fn new(toolchain: Toolchain, profile: Profile, dependencies: &'a [FetchedDependency]) -> Self {
        Self {
            toolchain,
            profile,
            dependencies,
        }
    }

    pub fn build(&self, include_path: &PathBuf, target_path: &TargetDependencyPath,) -> Result<()> {
        Ok(())
    }
}
