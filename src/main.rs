use crate::cli::{CommandCli, CompilationDatabaseArgs};
use clap::Parser;

mod config;
mod cli;
mod std_command_ext;
mod tool;
mod command;
mod extension_manager;

fn main() {
    simple_logger::SimpleLogger::new().env().init().unwrap();

    let args = cli::CLI::parse();

    match args.command {
        CommandCli::Init(init_args) => {
            let compdb_args = CompilationDatabaseArgs {
                path: init_args.path.clone()
            };

            command::init::init(init_args).unwrap();
            command::compilation_database::compilation_database(compdb_args).unwrap();
        },
        CommandCli::Clean(clean_args) => command::clean::clean(clean_args).unwrap(),
        CommandCli::Build(build_args) => command::build::build(build_args).unwrap(),
        CommandCli::Run(build_args) => command::run::run(build_args),
        CommandCli::Compdb(compdb_args) => command::compilation_database::compilation_database(compdb_args).unwrap(),
    }
}
