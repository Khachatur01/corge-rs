use crate::cli::CommandCli;
use clap::Parser;
use tools::command;

mod config;
mod cli;
mod tools;

fn main() {
    let args = cli::CLI::parse();

    match args.command {
        CommandCli::Init(init_args) => command::init::init(init_args),
        CommandCli::Clean(clean_args) => command::clean::clean(clean_args),
        CommandCli::Build(build_args) => command::build::build(build_args),
        CommandCli::Run(build_args) => command::run::run(build_args),
    }
}
