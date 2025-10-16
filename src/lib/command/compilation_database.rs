pub mod compilation_database_path;

use crate::cli::{BuildModeCli, CompilationDatabaseArgs};
use crate::command::build::dependency_path::DependencyPath;
use crate::config::{Config, OptimizationLevel, Profile};
use crate::tool::files_fetcher::fetch_files;
use anyhow::{bail, Context, Result};
use serde::Serialize;
use std::fs;
use crate::command::compilation_database::compilation_database_path::CompilationDatabasePath;

#[derive(Serialize)]
struct CompileCommand {
    directory: String,
    file: String,
    command: String,
}

pub fn compilation_database(compilation_database_args: CompilationDatabaseArgs) -> Result<()> {
    let project_path = compilation_database_args.path.clone();

    log::info!("Generating compilation database in directory {:?}", &project_path);

    let dependency_path = DependencyPath::create(&project_path)?;
    let compilation_database_path = CompilationDatabasePath::create(&project_path)?;

    // Collect project source files
    let source_files_paths = fetch_files(&project_path, "c").context("Failed to fetch source files for project")?;

    let project_path = fs::canonicalize(project_path)?;
    let include_path = fs::canonicalize(dependency_path.include)?;

    let compile_commands: Result<Vec<CompileCommand>> = source_files_paths
        .iter()
        .map(|source_file_path| {
            let source_file_path = fs::canonicalize(source_file_path)?;

            Ok(CompileCommand {
                directory: project_path.display().to_string(),
                file: source_file_path.display().to_string(),
                command: format!("gcc -c {} -I {}", source_file_path.display(), include_path.display())
            })
        }).collect();

    match compile_commands {
        Ok(compile_commands) => {
            let serialized = serde_json::to_string_pretty(&compile_commands)?;
            fs::write(compilation_database_path.json, serialized)?;
        }
        Err(err) => bail!("Failed to generate compile commands: {}", err)
    }

    log::info!("COMPILATION DATABASE GENERATED SUCCESSFULLY");
    Ok(())
}
