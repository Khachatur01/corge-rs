use crate::config::LinkStrategy;
use clap::{Parser, Subcommand};
use std::fmt::Display;
use std::path::PathBuf;

#[derive(Clone, Debug, Default)]
pub enum BuildModeCli {
    #[default]
    Development,
    Release,
}
impl Display for BuildModeCli {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            BuildModeCli::Release => "release".to_string(),
            BuildModeCli::Development => "development".to_string(),
        };
        write!(f, "{}", str)
    }
}

#[derive(Parser, Debug)]
pub struct InitArgs {
    #[arg(default_value = "./", value_name = "PATH")]
    pub path: PathBuf,

    /// Initializes the project as an executable (binary).
    #[arg(long, group = "link_strategy")]
    pub executable: bool,

    /// Initializes the project as a static library.
    #[arg(long, group = "link_strategy")]
    pub s_lib: bool,

    /// Initializes the project as a dynamic library.
    #[arg(long, group = "link_strategy")]
    pub d_lib: bool,
}

impl InitArgs {
    pub fn link_strategy(&self) -> LinkStrategy {
        match (self.executable, self.s_lib, self.d_lib) {
            (true, false, false) => LinkStrategy::Executable,
            (false, true, false) => LinkStrategy::StaticLibrary,
            (false, false, true) => LinkStrategy::DynamicLibrary,
            _ => LinkStrategy::Executable
        }
    }
}

#[derive(Parser, Debug)]
pub struct CleanArgs {
    #[arg(default_value = "./", value_name = "PATH")]
    pub path: PathBuf,

    /// Clean the dependencies directory too.
    #[arg(long)]
    pub deps_too: bool,
}

#[derive(Parser, Debug)]
pub struct BuildArgs {
    #[arg(default_value = "./", value_name = "PATH")]
    pub path: PathBuf,

    /// Selects the toolchain to use from the build.yaml file.
    #[arg(long, value_name = "TOOLCHAIN")]
    pub toolchain: Option<String>,

    /// Builds the project in release mode (optimized).
    #[arg(long, group = "build_mode")]
    pub release: bool,

    /// Builds the project in development mode (debug info).
    #[arg(long, group = "build_mode")]
    pub dev: bool,
}
impl BuildArgs {
    pub fn build_mode(&self) -> BuildModeCli {
        match (self.release, self.dev) {
            (true, false) => BuildModeCli::Release,
            (false, true) => BuildModeCli::Development,
            _ => BuildModeCli::Development
        }
    }
}


#[derive(Subcommand, Debug)]
pub enum CommandCli {
    /// Initializes a new project.
    Init(InitArgs),
    /// Cleans the project build directory and optionally dependencies directory.
    Clean(CleanArgs),
    /// Builds the project.
    Build(BuildArgs),
    /// Builds and runs the project.
    Run(BuildArgs),
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CLI {
    #[command(subcommand)]
    pub command: CommandCli,
}
