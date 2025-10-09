use anyhow::Result;
use std::path::PathBuf;
use crate::config::Toolchain;
use crate::tool::dependency_source_fetcher::Artifact;

pub struct Compiler {
    toolchain: Toolchain,
}

impl Compiler {
    pub fn new(toolchain: Toolchain) -> Self {
        Self {
            toolchain,
        }
    }

    pub fn compile(&self, artifact: &Artifact, include_path: &PathBuf, target_path: &PathBuf,) -> Result<()> {
        println!("Compiling {} with include dir {} into {}", artifact.dependency.name, include_path.display(), target_path.display());
        Ok(())
    }
}