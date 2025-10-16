use crate::config::Dependency;
use crate::tool::dir_copier::deep_copy;
use anyhow::Result;
use std::path::Path;

pub fn fetch_fs_dependency(
    repository_path: &Path,
    dependency: &Dependency,
    artifact_path: &Path
) -> Result<()> {
    let dependency_path = repository_path.join(&dependency.name);

    deep_copy(dependency_path, artifact_path)?;

    Ok(())
}
