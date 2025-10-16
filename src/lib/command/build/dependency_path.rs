use anyhow::{Context, Result};
use std::path::PathBuf;

pub struct DependencyPath {
    pub source: PathBuf,
    pub include: PathBuf,
}

impl DependencyPath {
    pub fn create(project_path: &PathBuf) -> Result<Self> {
        let dependency_path = project_path.join("dependency");

        let this = Self {
            source: dependency_path.join("source"),
            include: dependency_path.join("include"),
        };

        std::fs::create_dir_all(&this.source)
            .context("Failed to create dependencies source directory")?;
        std::fs::create_dir_all(&this.include)
            .context("Failed to create dependencies include directory")?;

        Ok(this)
    }
}
