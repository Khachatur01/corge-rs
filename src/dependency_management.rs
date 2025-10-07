mod fs_repository;
mod git_repository;

use crate::dependency_management::fs_repository::copy_fs_dependency;
use crate::dependency_management::git_repository::copy_git_dependency;
use crate::model::{Config, Dependency, Repository};
use std::collections::HashMap;
use std::path::Path;
use std::fs;

fn copy_headers(src_path: &Path, dst_path: &Path) {
    let source_dir = fs::read_dir(src_path).expect(&format!("Can't read directory {:?}", src_path));

    for file in source_dir {
        let file = file.unwrap();

        if file.path().is_dir() {
            copy_headers(file.path().as_path(), dst_path.join(file.file_name()).as_ref());
        } else {
            file.path()
                .extension()
                .filter(|extension| *extension == "h")
                .iter()
                .for_each(|_| {
                    fs::create_dir_all(dst_path).unwrap();
                    fs::copy(file.path(), dst_path.join(file.file_name())).unwrap();
                });
        }
    }
}

fn copy_dependency(registry: &Repository, dependency: &Dependency, output_directory: &Path) -> Config {
    match registry {
        Repository::Git { url, branch } => copy_git_dependency(
            url,
            branch,
            dependency,
            output_directory
        ),
        Repository::FileSystem(registry_path) => copy_fs_dependency(
            registry_path.as_ref(),
            dependency,
            output_directory
        ),
    }
}

pub fn resolve_dependencies(repositories: &HashMap<String, Repository>, dependencies: &[Dependency], output_directory: &Path) {
    for dependency in dependencies {
        let Some(repository) = repositories.get(&dependency.repository_name) else {
            panic!("Registry {} not found", dependency.repository_name);
        };

        let config = copy_dependency(repository, &dependency, output_directory);

        if let Some(dependencies) = &config.dependencies {
            resolve_dependencies(
                &config.repositories.unwrap_or_default(),
                &dependencies,
                output_directory
            );
        }
    }
}
