use anyhow::Context;
use std::fs;
use std::path::PathBuf;

pub struct TargetCachePath {
    pub project: PathBuf,
    pub dependency: PathBuf,
}

pub struct TargetToolchainPath {
    pub cache: TargetCachePath,
    pub output: PathBuf,
}

pub struct TargetBuildModePath {
    pub toolchain: TargetToolchainPath,
}

pub struct TargetPath {
    pub build_mode: TargetBuildModePath,
}

impl TargetPath {
    pub fn create(project_path: &PathBuf, build_mode: &str, toolchain_name: &str) -> anyhow::Result<Self> {
        let toolchain_path = project_path.join("target").join(build_mode).join(toolchain_name);
        let cache_path = toolchain_path.join("cache");

        let this = Self {
            build_mode: TargetBuildModePath {
                toolchain: TargetToolchainPath {
                    cache: TargetCachePath {
                        project: cache_path.join("project"),
                        dependency: cache_path.join("dependency"),
                    },
                    output: toolchain_path.join("output"),
                }
            }
        };

        fs::create_dir_all(&this.build_mode.toolchain.cache.project)
            .context("Failed to create target project cache directory")?;
        fs::create_dir_all(&this.build_mode.toolchain.cache.dependency)
            .context("Failed to create target dependency cache directory")?;
        fs::create_dir_all(&this.build_mode.toolchain.output)
            .context("Failed to create target output directory")?;

        Ok(this)
    }
}
