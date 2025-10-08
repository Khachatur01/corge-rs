use crate::config::{Dependency, Registry};

pub struct DependencySource {
    registry: Registry,
    dependency: Dependency,
}

/**
    Converts a dependency tree into a flat dependency list.
 */
pub struct DependencyResolver {
    registries: Vec<Registry>,
    dependencies: Vec<Dependency>,
}

impl DependencyResolver {
    pub fn new(registries: Vec<Registry>, dependencies: Vec<Dependency>,) -> Self {
        Self {
            registries,
            dependencies,
        }
    }

    pub fn resolve(&self) -> Vec<DependencySource> {
        todo!()
    }
}
