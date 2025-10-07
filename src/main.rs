use crate::build::compiler::Compiler;
use crate::cli::{BuildArgs, BuildModeCli, CleanArgs, CommandCli, InitArgs};
use crate::config::{Toolchain, Config, OptimizationLevel, Profile};
use clap::Parser;
use std::fs;
use std::path::PathBuf;

mod config;
mod cli;
mod package_manager;
mod build;
mod extension;

fn init(init_args: InitArgs) {
    println!("Initializing project in directory {:?}", init_args.path);
    if fs::exists(&init_args.path).unwrap() {
        panic!("Directory {:?} already exists", init_args.path);
    }

    fs::create_dir_all(&init_args.path).unwrap();

    fs::create_dir_all(init_args.path.join("src")).unwrap();
    fs::write(init_args.path.join("src").join("main.c"), "").unwrap();
    fs::write(init_args.path.join(".gitignore"), "").unwrap();
    fs::write(init_args.path.join("build.yaml"), "").unwrap();
}

fn clean(clean_args: CleanArgs) {
    fs::remove_dir_all(clean_args.path.join("target")).unwrap();

    if clean_args.deps_too {
        fs::remove_dir_all(clean_args.path.join("dependency")).unwrap();
    }
}

fn build(build_args: BuildArgs) {
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

fn run(build_args: BuildArgs) {
    println!("Running project in directory {:?}", build_args)
}

fn main() {
    let args = cli::CLI::parse();

    match args.command {
        CommandCli::Init(init_args) => init(init_args),
        CommandCli::Clean(clean_args) => clean(clean_args),
        CommandCli::Build(build_args) => build(build_args),
        CommandCli::Run(build_args) => run(build_args),
    }
}
