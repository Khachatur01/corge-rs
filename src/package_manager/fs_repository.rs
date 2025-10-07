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

pub fn copy_fs_dependency(repository_path: &Path, dependency: &Dependency, output_directory: &Path) -> Option<Config> {
    println!("Copying dependency '{}' from 'fs' repository {:?}", dependency.name, repository_path);

    /* todo: check if dependency already exists */

    let header_dir = &output_directory.join("header").join(&dependency.name);
    let source_dir = &output_directory.join("source").join(&dependency.name);

    let dependency_path = repository_path.join(&dependency.name);
    copy_dir_all(dependency_path, source_dir).unwrap();

    copy_headers(&source_dir.join("src"), header_dir);

    fs::read_to_string(source_dir.join("build.yaml"))
        .ok()
        .map(|config_str: String| serde_yaml::from_str(&config_str))?
        .ok()
}
