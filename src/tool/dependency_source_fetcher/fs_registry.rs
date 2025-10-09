use crate::config::Dependency;
use std::path::Path;
use std::{fs, io};
use anyhow::{Context, Result};

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;

        if entry.file_type()?.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

pub fn fetch_fs_dependency(
    repository_path: &Path,
    dependency: &Dependency,
    artifact_path: &Path
) -> Result<()> {
    let dependency_path = repository_path.join(&dependency.name);

    copy_dir_all(dependency_path, artifact_path)?;

    Ok(())
}
