use crate::build::compiler::Compiler;
use crate::command_line_arguments::CommandCli;
use crate::command_line_arguments::ProfileCli;
use crate::package_manager::resolve_dependencies;
use crate::config::Config;
use clap::Parser;
use std::fs;
use std::path::PathBuf;

mod config;
mod command_line_arguments;
mod package_manager;
mod build;
mod extension;

fn build(project_dir: &PathBuf, profile_cli: &ProfileCli) {
    println!("Building project in directory {}", project_dir.display().to_string());
    let config_str: String = fs::read_to_string(project_dir.join("build.yaml")).unwrap();

    let config: Config = serde_yaml::from_str(&config_str).unwrap();

    let _ = fs::create_dir_all(project_dir.join("dependency"));

    println!("Resolving dependencies...");
    if let Some(dependencies) = &config.dependencies {
        resolve_dependencies(
            &config.repositories.unwrap_or_default(),
            &dependencies,
            project_dir.join("dependency").as_path()
        );
    }

    let builder = config.builder.unwrap_or_default();

    Compiler::new(
        project_dir.clone(),
        project_dir.join("dependency").join("header").clone(),
        project_dir.join("target").clone(),

        config.project.name,
        config.project.project_type,

        builder.compiler,
        builder.archiver,
        builder.compiler_flags,
        builder.linker_flags,
    ).compile(
        profile_cli
    );
}

fn run(project_dir: &str, profile: &ProfileCli) {
    println!("Running project in directory {}", project_dir)
}

fn main() {
    let args = command_line_arguments::CommandLineArguments::parse();

    match args.command {
        CommandCli::Init => todo!("Generate a new project with 'src' directory, 'build.yaml' and .gitignore file"),
        CommandCli::Clean => todo!("Delete dependency and target directory"),
        CommandCli::Build { profile } => build(&PathBuf::from(args.project_dir), &profile.unwrap_or_default()),
        CommandCli::Run { profile } => run(&args.project_dir, &profile.unwrap_or_default()),
    }
}
