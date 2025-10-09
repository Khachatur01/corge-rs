pub mod target_path;
pub mod dependency_path;

use anyhow::{Context, Result};
use crate::cli::{BuildArgs, BuildModeCli};
use crate::command::build::dependency_path::DependencyPath;
use crate::command::build::target_path::TargetPath;
use crate::config::{Config, OptimizationLevel, Profile};
use crate::tool::configuration_parser::ConfigurationParser;
use crate::tool::dependencies_compiler::DependenciesCompiler;
use crate::tool::dependency_include_fetcher::DependencyIncludeFetcher;
use crate::tool::dependency_source_fetcher::DependencySourceFetcher;

pub fn build(build_args: BuildArgs) -> Result<()> {
    let project_path = build_args.path.clone();

    log::info!("Building project in directory {:?}", &project_path);

    /** Configuration parsing */
    let config = ConfigurationParser::new(project_path.clone())
        .parse()
        .context("Corge project is not detected")?;

    let build_mode = build_args.build_mode();
    let profile = fetch_profile(&config, &build_mode);
    let (toolchain_name, toolchain) = config.toolchain(build_args.toolchain)
        .context("Failed to find toolchain in build.yaml file")?;

    /* Path definition */
    let dependency_path = DependencyPath::create(&project_path)?;
    let target_path = TargetPath::create(&project_path, &build_mode.to_string(), &toolchain_name)?;

    /** Dependency fetching */
    let dependencies = DependencySourceFetcher::new(config.registries, config.dependencies, )
        .fetch(&dependency_path.source)
        .context("Failed to fetch dependency sources")?;

    DependencyIncludeFetcher::new(&dependencies)
        .fetch(&dependency_path.include)
        .context("Failed to fetch dependency headers")?;
    

    /** Dependency building */
    DependenciesCompiler::new(toolchain, profile, &dependencies)
        .compile(
            &dependency_path.include,
            &target_path.build_mode.toolchain.cache.dependency
        )
        .context("Failed to build dependencies")?;

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


// pub fn build(build_args: BuildArgs) -> Result<()> {
//     log::info!("Building project in directory {:?}", build_args.path);
//
//     let config_str: String = fs::read_to_string(build_args.path.join("build.yaml"))
//         .context("Failed to read 'build.yaml' file")?;
//
//     let mut config: Config = serde_yaml::from_str(&config_str)
//         .context("Failed to parse 'build.yaml' file")?;
//
//     fs::create_dir_all(build_args.path.join("dependency"))
//         .context("Failed to create 'dependency' directory")?;
//
//     let (toolchain_name, toolchain) = config.toolchain(build_args.toolchain.clone())
//         .context("Failed to find toolchain in build.yaml file")?;
//
//     let project_dir: PathBuf = build_args.path.clone();
//     let build_mode: BuildModeCli = build_args.build_mode();
//
//     log::info!("Fetching dependencies...");
//     dependency_manager::fetch_dependencies(
//         &config.repositories,
//         &config.dependencies,
//         &build_args.path.join("dependency")
//     ).context("Failed to fetch dependencies")?;
//
//     let manifests: Vec<Manifest> = get_dependency_manifests(&project_dir, &config.dependencies);
//
//     let profile = match build_mode {
//         BuildModeCli::Development => {
//             config.profiles.unwrap_or_default().development.unwrap_or_else(|| Profile {
//                 optimization_level: OptimizationLevel::O
//             })
//         }
//         BuildModeCli::Release => {
//             config.profiles.unwrap_or_default().release.unwrap_or_else(|| Profile {
//                 optimization_level: OptimizationLevel::Ofast
//             })
//         }
//     };
//
//     Compiler::new(
//         project_dir.clone(),
//         project_dir.join("dependency").join("header").clone(),
//         project_dir.join("target").join(toolchain_name).clone(),
//
//         config.project.name,
//         config.project.project_type,
//
//         manifests,
//
//         toolchain
//     ).compile(
//         &build_mode,
//         &profile
//     );
//
//     Ok(())
// }
