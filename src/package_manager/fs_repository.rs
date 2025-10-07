use std::{fs, io};
use std::path::Path;
use crate::package_manager::copy_headers;
use crate::config::{Config, Dependency, ProjectType};

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;

        if entry.file_type()?.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

pub fn copy_fs_dependency(repository_path: &Path, dependency: &Dependency, output_directory: &Path) -> Config {
    println!("Copying dependency '{}' from 'fs' repository {:?}", dependency.name, repository_path);

    /* todo: check if dependency already exists */

    let dependency_path = repository_path.join(&dependency.name);
    let src_path = dependency_path.join("src");

    let config_str = fs::read_to_string(dependency_path.join("build.yaml")).unwrap();
    let config: Config = serde_yaml::from_str(&config_str).unwrap();

    if let ProjectType::Executable = &config.project.project_type {
        panic!("Executable dependencies are not supported");
    }

    copy_dir_all(dependency_path, output_directory.join("source").join(&dependency.name)).unwrap();

    copy_headers(src_path.as_path(), output_directory.join("header").join(&dependency.name).as_path());

    config
}
