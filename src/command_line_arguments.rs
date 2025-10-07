use std::fmt::Display;
use clap::{Parser, Subcommand};

#[derive(Subcommand, Clone, Debug, Default)]
pub enum ProfileCli {
    Release,
    #[default]
    Development,
}

impl Display for ProfileCli {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            ProfileCli::Release => "release".to_string(),
            ProfileCli::Development => "development".to_string(),
        };
        write!(f, "{}", str)
    }
}

#[derive(Subcommand, Debug)]
pub enum CommandCli {
    Init,
    Clean,
    Build {
        #[command(subcommand)]
        profile: Option<ProfileCli>,
    },
    Run {
        #[command(subcommand)]
        profile: Option<ProfileCli>,
    },
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CommandLineArguments {
    #[command(subcommand)]
    pub command: CommandCli,

    #[arg(long, allow_hyphen_values(true), default_value_t = String::from("./"))]
    pub project_dir: String,
}
