use crate::cli::CleanArgs;
use anyhow::{Context, Result};
use std::fs;

pub fn clean(clean_args: CleanArgs) -> Result<()> {
    fs::remove_dir_all(clean_args.path.join("target"))
        .with_context(|| format!("Failed to remove directory {:?}", &clean_args.path.join("target")))?;

    if clean_args.deps_too {
        fs::remove_dir_all(clean_args.path.join("dependency"))
            .with_context(|| format!("Failed to remove directory {:?}", &clean_args.path.join("dependency")))?;
    }

    Ok(())
}
