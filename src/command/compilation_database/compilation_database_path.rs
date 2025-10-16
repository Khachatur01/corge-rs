use anyhow::{Context, Result};
use std::path::PathBuf;

pub struct CompilationDatabasePath {
    pub json: PathBuf,
}

impl CompilationDatabasePath {
    pub fn create(project_path: &PathBuf) -> Result<Self> {
        let compilation_database_path = project_path.join("compilation_database");

        let this = Self {
            json: compilation_database_path.join("compile_commands.json")
        };

        std::fs::create_dir_all(&compilation_database_path)
            .context("Failed to create compilation database directory")?;

        Ok(this)
    }
}
