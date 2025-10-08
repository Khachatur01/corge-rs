use std::fs;
use std::path::{Path, PathBuf};

pub fn find_source_files(directory: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();

    for file in fs::read_dir(directory).unwrap() {
        let file = file.unwrap();

        if file.path().is_dir() {
            let inner_files = find_source_files(file.path().as_path());
            files.extend(inner_files);
        } else if let Some(extension) = file.path().extension() && extension == "c" {
            files.push(file.path());
        }
    }

    files
}
