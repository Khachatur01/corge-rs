use std::fs;
use std::path::PathBuf;
use crate::compiler::Compiler;
use crate::cli::{BuildArgs, BuildModeCli};
use crate::config::{Config, OptimizationLevel, Profile, Toolchain};
use crate::package_manager;

pub fn build(build_args: BuildArgs) {
    println!("Building project in directory {:?}", build_args.path);

    let config_str: String = fs::read_to_string(build_args.path.join("build.yaml")).unwrap();

    let config: Config = serde_yaml::from_str(&config_str).unwrap();

    let _ = fs::create_dir_all(build_args.path.join("dependency"));

    println!("Resolving dependencies...");
    if let Some(dependencies) = &config.dependencies {
        package_manager::resolve_dependencies(
            &config.repositories.unwrap_or_default(),
            &dependencies,
            build_args.path.join("dependency").as_path()
        );
    }

    /* remove toolchain from config to get it to avoid cloning it */
    let (toolchain_name, toolchain) =
        match &build_args.toolchain {
            None => ("default".to_string(), Toolchain::default()),
            Some(toolchain_name) => (toolchain_name.clone(), config.toolchains.unwrap_or_default().remove(toolchain_name).unwrap()),
        };

    let project_dir: PathBuf = build_args.path.clone();
    let build_mode: BuildModeCli = build_args.into();

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

        toolchain
    ).compile(
        &build_mode,
        &profile
    );
}
