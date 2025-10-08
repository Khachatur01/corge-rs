use crate::cli::{BuildArgs, BuildModeCli};
use crate::config::{Config, OptimizationLevel, Profile};
use crate::tools::compiler::Compiler;
use crate::tools::manifest_manager::{get_dependency_manifests, Manifest};
use crate::tools::dependency_manager;
use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

pub fn build(build_args: BuildArgs) -> Result<()> {
    log::info!("Building project in directory {:?}", build_args.path);

    let config_str: String = fs::read_to_string(build_args.path.join("build.yaml"))
        .context("Failed to read 'build.yaml' file")?;

    let mut config: Config = serde_yaml::from_str(&config_str)
        .context("Failed to parse 'build.yaml' file")?;

    fs::create_dir_all(build_args.path.join("dependency"))
        .context("Failed to create 'dependency' directory")?;

    let (toolchain_name, toolchain) = config.toolchain(build_args.toolchain.clone())
        .context("Failed to find toolchain in build.yaml file")?;

    let project_dir: PathBuf = build_args.path.clone();
    let build_mode: BuildModeCli = build_args.build_mode();

    log::info!("Fetching dependencies...");
    dependency_manager::fetch_dependencies(
        &config.repositories,
        &config.dependencies,
        &build_args.path.join("dependency")
    ).context("Failed to fetch dependencies")?;

    let manifests: Vec<Manifest> = get_dependency_manifests(&project_dir, &config.dependencies);

    let profile = match build_mode {
        BuildModeCli::Development => {
            config.profiles.unwrap_or_default().development.unwrap_or_else(|| Profile {
                optimization_level: OptimizationLevel::O
            })
        }
        BuildModeCli::Release => {
            config.profiles.unwrap_or_default().release.unwrap_or_else(|| Profile {
                optimization_level: OptimizationLevel::Ofast
            })
        }
    };

    Compiler::new(
        project_dir.clone(),
        project_dir.join("dependency").join("header").clone(),
        project_dir.join("target").join(toolchain_name).clone(),

        config.project.name,
        config.project.project_type,

        manifests,

        toolchain
    ).compile(
        &build_mode,
        &profile
    );

    Ok(())
}
