pub mod configuration_parser;
pub mod dependency_source_fetcher;
pub mod dependency_include_fetcher;
pub mod compiler;
pub mod linker;
pub mod files_fetcher;
// use crate::config::{Dependency, Registry};

// pub struct ConfigurationParser {
//     project_directory: String,
// }
//
// pub struct DependencyResolver {
//     registries: Vec<Registry>,
//     dependencies: Vec<Dependency>,
// }
//
// pub struct DependencyFetcher {
//     registries: Vec<Registry>,
//     dependencies: Vec<Dependency>,
// }
//
// pub struct Builder {
// }
//
// pub enum Stage {
//     ParseConfiguration(ConfigurationParser),
//     ResolveDependencies(DependencyResolver),
//     FetchDependencies(DependencyFetcher),
//     Build(Builder),
// }
