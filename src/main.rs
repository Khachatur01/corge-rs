use crate::build::compiler::Compiler;
use crate::command_line_arguments::CommandCli;
use crate::command_line_arguments::ProfileCli;
use crate::dependency_management::resolve_dependencies;
use crate::model::Config;
use clap::Parser;
use std::fs;
use std::path::PathBuf;

mod model;
mod command_line_arguments;
mod dependency_management;
mod build;
mod extension;

fn build(project_directory: &PathBuf, profile_cli: &ProfileCli) {
    println!("Building project in directory {}", project_directory.display().to_string());
    let config_str: String = fs::read_to_string(project_directory.join("build.yaml")).unwrap();

    let config: Config = serde_yaml::from_str(&config_str).unwrap();

    let _ = fs::create_dir_all(project_directory.join("dependency"));

    println!("Resolving dependencies...");
    if let Some(dependencies) = &config.dependencies {
        resolve_dependencies(
            &config.repositories.unwrap_or_default(),
            &dependencies,
            project_directory.join("dependency").as_path()
        );
    }

    let builder = config.builder.unwrap_or_default();

    Compiler::new(
        project_directory.clone(),
        project_directory.join("dependency").join("header").clone(),
        project_directory.join("target").clone(),

        builder.compiler,
        builder.archiver,
        builder.compiler_flags,
        builder.linker_flags,
    ).compile(
        &config.project
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
