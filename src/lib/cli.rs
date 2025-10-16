use crate::config::LinkStrategy;
use clap::{Parser, Subcommand};
use std::fmt::Display;
use std::path::PathBuf;

#[derive(Debug, Default, Clone)]
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

#[derive(Parser, Debug, Clone)]
pub struct InitArgs {
    #[arg(default_value = "./", value_name = "PATH")]
    pub path: PathBuf,

    /// Initializes the project without a git repository.
    #[arg(long, default_value = "false", value_name = "NO_GIT")]
    pub no_git: bool,
}

#[derive(Parser, Debug, Clone)]
pub struct CleanArgs {
    #[arg(default_value = "./", value_name = "PATH")]
    pub path: PathBuf,

    /// Clean the dependencies directory too.
    #[arg(long)]
    pub deps_too: bool,
}

#[derive(Subcommand, Debug, Clone)]
pub enum CloneSource {
    Git {
        #[arg(value_name = "URL")]
        url: String,
        #[arg(long, default_value = "master", value_name = "BRANCH")]
        branch: String,
    },
    FileSystem {
        #[arg(value_name = "PATH")]
        from: PathBuf,
    },
}

#[derive(Parser, Debug, Clone)]
pub struct CloneArgs {
    #[arg(default_value = "./", value_name = "PATH")]
    pub path: PathBuf,

    #[clap(subcommand)]
    pub source: CloneSource,
}

#[derive(Parser, Debug, Clone)]
pub struct CompilationDatabaseArgs {
    #[arg(default_value = "./", value_name = "PATH")]
    pub path: PathBuf,
}

#[derive(Subcommand, Debug, Clone)]
pub enum BuildToolchain {
    /// Selects the default toolchain.
    Default,
    /// Selects toolchain by name from the build.yaml file
    Named {
        #[arg(value_name = "NAME")]
        name: String,
    },
    /// Selects a custom toolchain.
    Custom {
        #[arg(long, default_value = "./", default_value = "gcc", value_name = "COMPILER")]
        compiler: String,
        #[arg(long, default_value = "./", default_value = "ar", value_name = "ARCHIVER")]
        archiver: String,
        #[clap(short, long, value_parser, num_args = 0.., value_delimiter = ' ')]
        compiler_flags: Vec<String>,
        #[clap(short, long, value_parser, num_args = 0.., value_delimiter = ' ')]
        linker_flags: Vec<String>,
    }
}

#[derive(Parser, Debug, Clone)]
pub struct BuildArgs {
    #[arg(default_value = "./", value_name = "PATH")]
    pub path: PathBuf,

    #[command(subcommand)]
    pub toolchain: Option<BuildToolchain>,

    /// Specifies the link strategy.
    #[arg(long, value_enum, default_value = "executable", value_name = "LINK_STRATEGY",)]
    pub link: LinkStrategy,

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


#[derive(Subcommand, Debug, Clone)]
pub enum CommandCli {
    /// Clone a project
    Clone(CloneArgs),
    /// Initializes a new project.
    Init(InitArgs),
    /// Cleans the project build directory and optionally dependencies directory.
    Clean(CleanArgs),
    /// Builds the project.
    Build(BuildArgs),
    /// Builds and runs the project.
    Run(BuildArgs),
    /// Generates a compile_commands.json for the project.
    Compdb(CompilationDatabaseArgs),
}

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct CLI {
    #[command(subcommand)]
    pub command: CommandCli,
}
