pub mod target_path;
pub mod dependency_path;

use crate::cli::{BuildArgs, BuildModeCli};
use crate::command::build::dependency_path::DependencyPath;
use crate::command::build::target_path::TargetPath;
use crate::config::{Config, LinkStrategy, OptimizationLevel, Profile};
use crate::tool::compiler::Compiler;
use crate::tool::configuration_parser::ConfigurationParser;
use crate::tool::dependency_include_fetcher::DependencyIncludeFetcher;
use crate::tool::dependency_source_fetcher::DependencySourceFetcher;
use crate::tool::files_fetcher::fetch_files;
use crate::tool::linker::Linker;
use anyhow::{Context, Result};
use std::fs;

pub fn build(build_args: BuildArgs) -> Result<()> {
    let project_path = build_args.path.clone();

    log::info!("Building project in directory {:?}", &project_path);

    /** Configuration parsing */
    log::info!("Parsing build.yaml file");
    let config = ConfigurationParser::new(project_path.clone())
        .parse()
        .context("Corge project is not detected")?;

    let build_mode = build_args.build_mode();
    let profile = fetch_profile(&config, &build_mode);
    let (toolchain_name, toolchain) = config.toolchain(build_args.subcommand)
        .context("Failed to find toolchain in build.yaml file")?;

    /* Path definition */
    let dependency_path = DependencyPath::create(&project_path)?;
    let target_path = TargetPath::create(&project_path, &build_mode.to_string(), &toolchain_name)?;

    /** Dependency fetching */
    log::info!("Fetching dependencies");
    let artifacts = DependencySourceFetcher::new(config.registries, config.dependencies, )
        .fetch(&dependency_path.source)
        .context("Failed to fetch dependency sources")?;

    DependencyIncludeFetcher::new(&artifacts)
        .fetch(&dependency_path.include)
        .context("Failed to fetch dependency headers")?;

    /** Compilation */
    log::info!("Compiling project");
    let compiler = Compiler::new(profile, toolchain.clone(), dependency_path.include.clone());

    /* generate position-independent code if the project is a dynamic library */
    let pic = matches!(config.project.link_strategy, LinkStrategy::DynamicLibrary);

    let mut object_files = vec![];

    /* compile dependencies artifacts */
    for artifact in &artifacts {
        let target_path = target_path.build_mode.toolchain.cache.dependency.join(&artifact.dependency.name);
        fs::create_dir_all(&target_path)
            .with_context(|| format!("Failed to create directory {:?}", &target_path))?;

        let source_files = fetch_files(&artifact.path, "c")
            .with_context(|| format!("Failed to fetch source files for dependency {}", &artifact.dependency.name))?;

        let artifact_object_files = compiler
            .compile(&source_files, &target_path, pic)
            .with_context(|| format!("Failed to compile dependency '{}' artifact", &artifact.dependency.name))?;

        object_files.extend(artifact_object_files);
    }

    /* compile project sources */
    let source_files = fetch_files(&project_path, "c")
        .context("Failed to fetch source files for project")?;

    let project_object_files = compiler
        .compile(&source_files, &target_path.build_mode.toolchain.cache.project, pic)
        .context("Failed to compile project files")?;
    object_files.extend(project_object_files);

    /** Linking */
    log::info!("Linking project");
    let linker = Linker::new(toolchain);
    linker.link(&config.project.link_strategy, &object_files, &target_path.build_mode.toolchain.output, &config.project.name)
        .context("Failed to link project")?;

    log::info!("BUILD SUCCESSFUL");
    Ok(())
}

fn fetch_profile(config: &Config, build_mode: &BuildModeCli,) -> Profile {
    match build_mode {
        BuildModeCli::Development => {
            config.profiles.development.clone().unwrap_or_else(|| Profile {
                optimization_level: OptimizationLevel::O
            })
        }
        BuildModeCli::Release => {
            config.profiles.release.clone().unwrap_or_else(|| Profile {
                optimization_level: OptimizationLevel::Ofast
            })
        }
    }
}
