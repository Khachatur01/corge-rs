use crate::cli::CleanArgs;
use anyhow::{Context, Result};
use std::fs;

pub fn clean(clean_args: CleanArgs) -> Result<()> {
    let target_directory = clean_args.path.join("target");
    let dependency_directory = clean_args.path.join("dependency");

    if target_directory.exists() {
        fs::remove_dir_all(&target_directory)
            .with_context(|| format!("Failed to remove directory {:?}", &target_directory))?;
    }

    if dependency_directory.exists() && clean_args.deps_too {
        fs::remove_dir_all(&dependency_directory)
            .with_context(|| format!("Failed to remove directory {:?}", &dependency_directory))?;
    }

    Ok(())
}
