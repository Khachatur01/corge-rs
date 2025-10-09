use crate::config::{Profile, Toolchain};
use crate::tool::compiler::Compiler;
use crate::tool::dependency_source_fetcher::Artifact;
use anyhow::{Context, Result};
use std::path::PathBuf;

pub struct DependenciesCompiler<'a> {
    toolchain: Toolchain,
    profile: Profile,
    artifacts: &'a [Artifact]
}

impl<'a> DependenciesCompiler<'a> {
    pub fn new(toolchain: Toolchain, profile: Profile, artifacts: &'a [Artifact]) -> Self {
        Self {
            toolchain,
            profile,
            artifacts,
        }
    }

    pub fn compile(&self, include_path: &PathBuf, target_path: &PathBuf,) -> Result<()> {
        for artifact in self.artifacts {
            Compiler::new(self.toolchain.clone())
                .compile(&artifact, include_path, &target_path.join(&artifact.dependency.name))
                .with_context(|| format!("Failed to compile dependency {}", &artifact.dependency.name))?;
        }

        Ok(())
    }
}
