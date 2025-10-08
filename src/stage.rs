pub mod parse_configuration;
mod resolve_dependencies;
mod fetch_dependencies;
mod builder;
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
